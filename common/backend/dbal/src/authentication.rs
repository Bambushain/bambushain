use sea_orm::prelude::Expr;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, NotSet, QueryFilter,
};

use bamboo_common_core::entities::*;
use bamboo_common_core::error::*;

use crate as dbal;
use crate::{decrypt_string, encrypt_string};

pub async fn create_token(username: String, db: &DatabaseConnection) -> BambooResult<LoginResult> {
    let user = crate::user::get_user_by_email_or_username(username.clone(), db)
        .await
        .map_err(|err| {
            log::error!("Failed to load user {username}: {err}");
            BambooError::not_found("user", "User not found")
        })?;

    token::ActiveModel {
        id: NotSet,
        token: Set(uuid::Uuid::new_v4().to_string()),
        user_id: Set(user.id),
    }
    .insert(db)
    .await
    .map(|token| LoginResult {
        token: token.token,
        user,
    })
    .map_err(|err| {
        log::error!("{err}");
        BambooError::database("token", "Failed to create token")
    })
}

pub async fn validate_auth(
    username: String,
    password: String,
    two_factor_code: Option<String>,
    db: &DatabaseConnection,
) -> BambooResult<TwoFactorResult> {
    let user = crate::user::get_user_by_email_or_username(username.clone(), db)
        .await
        .map_err(|err| {
            log::error!("Failed to load user {username}: {err}");
            BambooError::not_found("user", "User not found")
        })?;

    let password_valid = user.validate_password(password.clone());
    if !password_valid {
        return Err(BambooError::validation("token", "Password is invalid"));
    }

    let mut requires_two_factor_code =
        user.totp_secret.is_some() && user.totp_validated.unwrap_or(false);
    if requires_two_factor_code {
        if let Some(two_factor_code) = two_factor_code {
            validate_two_factor_code(user.id, two_factor_code, password, false, db).await?;
            requires_two_factor_code = false;
        }
    }

    Ok(TwoFactorResult {
        user,
        requires_two_factor_code,
    })
}

pub async fn delete_token(token: String, db: &DatabaseConnection) -> BambooErrorResult {
    bamboo_common_core::entities::token::Entity::delete_many()
        .filter(bamboo_common_core::entities::token::Column::Token.eq(token))
        .exec(db)
        .await
        .map(|_| ())
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("token", "Failed to delete the token")
        })
}

pub async fn validate_two_factor_code(
    id: i32,
    code: String,
    password: String,
    initial_validation: bool,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    let user = dbal::get_user(id, db).await?;

    let password_valid = user.validate_password(password.clone());
    if !password_valid {
        return Err(BambooError::unauthorized("user", "Invalid login data"));
    }

    if initial_validation || user.totp_validated.unwrap_or(false) {
        validate_totp_token(code, password, user, db).await
    } else {
        Ok(())
    }
}

async fn validate_totp_token(
    code: String,
    password: String,
    user: User,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    let totp_secret = if user.totp_secret_encrypted {
        decrypt_string(user.totp_secret.unwrap(), password.clone())?
    } else {
        let decrypted_secret = user.totp_secret.unwrap();
        let encrypted_secret = encrypt_string(decrypted_secret.clone(), password.clone())?;

        user::Entity::update_many()
            .col_expr(user::Column::TotpSecretEncrypted, Expr::value(true))
            .col_expr(user::Column::TotpSecret, Expr::value(encrypted_secret))
            .filter(user::Column::Id.eq(user.id))
            .exec(db)
            .await
            .map_err(|_| BambooError::database("user", "Failed to validate"))?;

        decrypted_secret
    };

    let is_totp_valid = totp_rs::TOTP::from_rfc6238(
        totp_rs::Rfc6238::new(
            6,
            totp_secret.clone(),
            Some("Bambushain".to_string()),
            user.display_name.clone(),
        )
        .map_err(|_| BambooError::crypto("user", "Failed to validate"))?,
    )
    .map_err(|err| {
        log::error!("Failed to create totp url {err}");
        BambooError::crypto("user", "Failed to validate")
    })
    .map(|totp| {
        totp.check_current(code.as_str()).unwrap_or_else(|err| {
            log::error!("Failed to validate totp {err}");
            false
        })
    })?;

    if is_totp_valid {
        Ok(())
    } else {
        Err(BambooError::crypto("user", "Failed to validate"))
    }
}

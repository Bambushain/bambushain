use base64::Engine;
use rand::distributions::Uniform;
use rand::Rng;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, NotSet, QueryFilter,
};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::Expr;

use bamboo_common_core::entities::*;
use bamboo_common_core::error::*;

use crate::{decrypt_string, encrypt_string};
use crate::prelude::dbal;
use crate::user::get_users;

pub async fn create_google_auth_token(
    password: String,
    db: &DatabaseConnection,
) -> BambooResult<LoginResult> {
    let user =
        crate::user::get_user_by_email_or_username("playstore@google.bambushain".to_string(), db)
            .await
            .map_err(|err| {
                log::error!("Failed to load user playstore@google.bambushain: {err}");
                BambooError::not_found("user", "User not found")
            })?;

    if !user.validate_password(password) {
        return Err(BambooError::unauthorized("user", "Invalid login data"));
    }

    token::ActiveModel {
        id: NotSet,
        token: Set(uuid::Uuid::new_v4().to_string()),
        user_id: Set(user.id),
    }
        .insert(db)
        .await
        .map(|token| LoginResult {
            token: token.token,
            user: user.clone().into(),
        })
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("token", "Failed to create token")
        })
}

pub async fn validate_auth_and_create_token(
    username: String,
    password: String,
    two_factor_code: String,
    db: &DatabaseConnection,
) -> BambooResult<LoginResult> {
    let user = crate::user::get_user_by_email_or_username(username.clone(), db)
        .await
        .map_err(|err| {
            log::error!("Failed to load user {username}: {err}");
            BambooError::not_found("user", "User not found")
        })?;

    validate_login(user.id, two_factor_code, password, false, db).await?;

    let result = token::ActiveModel {
        id: NotSet,
        token: Set(uuid::Uuid::new_v4().to_string()),
        user_id: Set(user.id),
    }
        .insert(db)
        .await
        .map(|token| LoginResult {
            token: token.token,
            user: user.clone().into(),
        })
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("token", "Failed to create token")
        });

    let _ = bamboo_common_core::entities::user::Entity::update_many()
        .col_expr(
            bamboo_common_core::entities::user::Column::TwoFactorCode,
            Expr::value::<Option<String>>(None),
        )
        .filter(bamboo_common_core::entities::user::Column::Id.eq(user.id))
        .exec(db)
        .await;

    result
}

pub async fn validate_auth_and_set_two_factor_code(
    username: String,
    password: String,
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

    if user.totp_secret.is_some() && user.totp_validated.unwrap_or(false) {
        return Ok(TwoFactorResult {
            user: user.into(),
            two_factor_code: None,
        });
    }

    let two_factor_code = rand::thread_rng()
        .sample_iter(&Uniform::new(0, 10))
        .take(6)
        .map(|id| id.to_string())
        .collect::<Vec<String>>()
        .join("");

    let encrypted_code = encrypt_string(two_factor_code.clone().into_bytes(), password)?;

    bamboo_common_core::entities::user::Entity::update_many()
        .col_expr(
            bamboo_common_core::entities::user::Column::TwoFactorCode,
            Expr::value(base64::prelude::BASE64_STANDARD.encode(encrypted_code)),
        )
        .filter(bamboo_common_core::entities::user::Column::Id.eq(user.id))
        .exec(db)
        .await
        .map_err(|_| BambooError::validation("token", "Failed to set two factor code"))
        .map(|_| TwoFactorResult {
            user: user.clone().into(),
            two_factor_code: Some(two_factor_code),
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

pub async fn get_tokens_by_grove(
    grove_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<Vec<Token>> {
    let users = get_users(grove_id, db).await?;
    let users = users.iter().map(|user| user.id);
    token::Entity::find()
        .filter(user::Column::Id.is_in(users))
        .inner_join(user::Entity)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("token", "Failed to get all tokens")
        })
}

pub async fn validate_login(
    id: i32,
    code: String,
    password: String,
    initial_validation: bool,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    let user = dbal::get_user_by_id_only(id, db).await?;

    let password_valid = user.validate_password(password.clone());
    if !password_valid {
        return Err(BambooError::unauthorized("user", "Invalid login data"));
    }

    if initial_validation || user.totp_validated.unwrap_or(false) {
        validate_totp_token(code, password, user, db).await
    } else {
        validate_email_token(code, password, user)
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

fn validate_email_token(code: String, password: String, user: User) -> BambooErrorResult {
    let two_factor_code = String::from_utf8_lossy(&decrypt_string(
        base64::prelude::BASE64_STANDARD
            .decode(user.two_factor_code.unwrap())
            .map_err(|_| BambooError::unauthorized("user", "Failed to validate"))?,
        password.clone(),
    )?)
        .into_owned();

    if two_factor_code.eq(&code) {
        Ok(())
    } else {
        Err(BambooError::unauthorized("user", "Failed to validate"))
    }
}

use base64::Engine;
use rand::distributions::Uniform;
use rand::Rng;
use sea_orm::prelude::Expr;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, NotSet, QueryFilter,
};

use bamboo_entities::prelude::*;
use bamboo_entities::{bamboo_db_error, bamboo_validation_error};

use crate::encrypt_string;
use crate::user::validate_login;

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
            bamboo_not_found_error!("user", "User not found")
        })?;

    validate_login(user.id, two_factor_code, password, false, db).await?;

    let result = bamboo_entities::token::ActiveModel {
        id: NotSet,
        token: Set(uuid::Uuid::new_v4().to_string()),
        user_id: Set(user.id),
    }
    .insert(db)
    .await
    .map(|token| LoginResult {
        token: token.token,
        user: user.to_web_user(),
    })
    .map_err(|err| {
        log::error!("{err}");
        bamboo_db_error!("token", "Failed to create token")
    });

    let _ = bamboo_entities::user::Entity::update_many()
        .col_expr(
            bamboo_entities::user::Column::TwoFactorCode,
            Expr::value::<Option<String>>(None),
        )
        .filter(bamboo_entities::user::Column::Id.eq(user.id))
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
            bamboo_not_found_error!("user", "User not found")
        })?;

    let password_valid = user.validate_password(password.clone());
    if !password_valid {
        return Err(bamboo_validation_error!("token", "Password is invalid"));
    }

    if user.totp_secret.is_some() && user.totp_validated.unwrap_or(false) {
        return Ok(TwoFactorResult {
            user: user.to_web_user(),
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

    bamboo_entities::user::Entity::update_many()
        .col_expr(
            bamboo_entities::user::Column::TwoFactorCode,
            Expr::value(base64::prelude::BASE64_STANDARD.encode(encrypted_code)),
        )
        .filter(bamboo_entities::user::Column::Id.eq(user.id))
        .exec(db)
        .await
        .map_err(|_| bamboo_validation_error!("token", "Failed to set two factor code"))
        .map(|_| TwoFactorResult {
            user: user.clone().to_web_user(),
            two_factor_code: Some(two_factor_code),
        })
}

pub async fn delete_token(token: String, db: &DatabaseConnection) -> BambooErrorResult {
    bamboo_entities::token::Entity::delete_many()
        .filter(bamboo_entities::token::Column::Token.eq(token))
        .exec(db)
        .await
        .map(|_| ())
        .map_err(|err| {
            log::error!("{err}");
            bamboo_db_error!("token", "Failed to delete the token")
        })
}

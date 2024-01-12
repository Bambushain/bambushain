use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use sea_orm::sea_query::Expr;

use bamboo_entities::prelude::*;
use bamboo_error::*;

use crate::{decrypt_string, encrypt_string};
use crate::prelude::dbal;

pub async fn change_my_password(
    id: i32,
    old_password: String,
    new_password: String,
    db: &DatabaseConnection,
) -> Result<(), PasswordError> {
    let hashed_password = bcrypt::hash(new_password.clone(), 12).map_err(|err| {
        log::error!("{err}");
        PasswordError::Unknown
    })?;

    let user = dbal::get_user_by_id_only(id, db)
        .await
        .map_err(|_| PasswordError::UserNotFound)?;
    let is_valid = user.validate_password(old_password.clone());

    if !is_valid {
        return Err(PasswordError::WrongPassword);
    }

    let (totp_secret, totp_secret_encrypted) = if user.totp_validated.unwrap_or(false) {
        let decrypted_totp_secret = if user.totp_secret_encrypted {
            decrypt_string(user.totp_secret.clone().unwrap(), old_password)
                .map_err(|_| PasswordError::Unknown)?
        } else {
            user.totp_secret.clone().unwrap()
        };

        let encrypted_totp_secret = encrypt_string(decrypted_totp_secret, new_password.clone())
            .map_err(|_| PasswordError::Unknown)?;

        (Some(encrypted_totp_secret), true)
    } else {
        (None, false)
    };

    user::Entity::update_many()
        .col_expr(user::Column::Password, Expr::value(hashed_password))
        .col_expr(
            user::Column::TotpSecretEncrypted,
            Expr::value(totp_secret_encrypted),
        )
        .col_expr(user::Column::TotpSecret, Expr::value(totp_secret))
        .filter(user::Column::Id.eq(id))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            PasswordError::Unknown
        })
        .map(|_| ())
}

pub async fn enable_my_totp(
    id: i32,
    secret: Vec<u8>,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    user::Entity::update_many()
        .col_expr(user::Column::TotpSecret, Expr::value(secret))
        .col_expr(user::Column::TotpSecretEncrypted, Expr::value(false))
        .col_expr(user::Column::TotpValidated, Expr::value(false))
        .filter(user::Column::Id.eq(id))
        .exec(db)
        .await
        .map_err(|_| BambooError::database("user", "The secret could not be saved"))
        .map(|_| ())
}

pub async fn disable_my_totp(id: i32, db: &DatabaseConnection) -> BambooErrorResult {
    user::Entity::update_many()
        .col_expr(
            user::Column::TotpSecret,
            Expr::value::<Option<Vec<u8>>>(None),
        )
        .col_expr(user::Column::TotpValidated, Expr::value(false))
        .col_expr(user::Column::TotpSecretEncrypted, Expr::value(false))
        .filter(user::Column::Id.eq(id))
        .exec(db)
        .await
        .map_err(|_| BambooError::database("user", "Failed to disable totp"))
        .map(|_| ())
}

pub async fn validate_my_totp(
    id: i32,
    password: String,
    code: String,
    db: &DatabaseConnection,
) -> BambooResult<bool> {
    let user = dbal::get_user_by_id_only(id, db).await?;
    let valid = dbal::validate_login(id, code, password.clone(), true, db)
        .await
        .is_ok();
    let totp_secret = encrypt_string(user.totp_secret.unwrap(), password)?;

    user::Entity::update_many()
        .col_expr(user::Column::TotpSecret, Expr::value(totp_secret))
        .col_expr(user::Column::TotpSecretEncrypted, Expr::value(true))
        .col_expr(user::Column::TotpValidated, Expr::value(Some(valid)))
        .filter(user::Column::Id.eq(id))
        .exec(db)
        .await
        .map_err(|_| BambooError::database("user", "Totp could not be validated"))
        .map(|_| valid)
}

pub async fn update_my_profile(
    id: i32,
    email: String,
    display_name: String,
    discord_name: String,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    if dbal::user_exists_by_id(id, email.clone(), display_name.clone(), db).await? {
        return Err(BambooError::exists_already(
            "user",
            "A user with that email or name exists already",
        ));
    }

    user::Entity::update_many()
        .col_expr(user::Column::Email, Expr::value(email))
        .col_expr(user::Column::DisplayName, Expr::value(display_name))
        .col_expr(user::Column::DiscordName, Expr::value(discord_name))
        .filter(user::Column::Id.eq(id))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("user", "Failed to update user")
        })
        .map(|_| ())
}

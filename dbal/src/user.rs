use base64::Engine;
use sea_orm::prelude::*;
use sea_orm::sea_query::Expr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel,
    JoinType, NotSet, QueryFilter, QueryOrder, QuerySelect, RelationTrait,
};

use bamboo_entities::prelude::*;
use bamboo_entities::user::Model;
use bamboo_entities::{
    bamboo_crypto_error, bamboo_db_error, bamboo_not_found_error, bamboo_unauthorized_error, token,
    user,
};

use crate::{decrypt_string, encrypt_string};

pub async fn get_user(id: i32, db: &DatabaseConnection) -> BambooResult<User> {
    match user::Entity::find_by_id(id).one(db).await {
        Ok(Some(res)) => Ok(res),
        Ok(None) => Err(bamboo_not_found_error!("user", "The user was not found")),
        Err(err) => {
            log::error!("{err}");
            Err(bamboo_db_error!("user", "Failed to execute database query"))
        }
    }
}

pub async fn get_user_by_email_or_username(
    username: String,
    db: &DatabaseConnection,
) -> BambooResult<User> {
    match user::Entity::find()
        .filter(
            Condition::any()
                .add(user::Column::Email.eq(username.clone()))
                .add(user::Column::DisplayName.eq(username)),
        )
        .one(db)
        .await
    {
        Ok(Some(res)) => Ok(res),
        Ok(None) => Err(bamboo_not_found_error!("user", "The user was not found")),
        Err(err) => {
            log::error!("{err}");
            Err(bamboo_db_error!("user", "Failed to execute database query"))
        }
    }
}

pub async fn get_users(db: &DatabaseConnection) -> BambooResult<Vec<User>> {
    user::Entity::find()
        .order_by_asc(user::Column::Email)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            bamboo_db_error!("user", "Failed to load users")
        })
}

pub async fn user_exists(id: i32, db: &DatabaseConnection) -> bool {
    user::Entity::find_by_id(id)
        .select_only()
        .column(user::Column::Id)
        .count(db)
        .await
        .map(|count| count > 0)
        .unwrap_or(false)
}

pub async fn create_user(user: User, db: &DatabaseConnection) -> BambooResult<User> {
    let mut model = user.into_active_model();
    model.id = NotSet;
    model
        .set_password(model.clone().password.as_ref())
        .map_err(|err| {
            log::error!("{err}");
            bamboo_db_error!("user", "Failed to hash password user")
        })?;

    model.insert(db).await.map_err(|err| {
        log::error!("{err}");
        bamboo_db_error!("user", "Failed to create user")
    })
}

pub async fn delete_user(id: i32, db: &DatabaseConnection) -> BambooErrorResult {
    user::Entity::delete_by_id(id)
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            bamboo_db_error!("user", "Failed to delete user")
        })
        .map(|_| ())
}

pub async fn change_mod_status(
    id: i32,
    is_mod: bool,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    user::Entity::update_many()
        .filter(user::Column::Id.eq(id))
        .col_expr(user::Column::IsMod, Expr::value(is_mod))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            bamboo_db_error!("user", "Failed to update user")
        })
        .map(|_| ())
}

pub async fn change_password(
    id: i32,
    password: String,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    let hashed_password = bcrypt::hash(password, 12).map_err(|err| {
        log::error!("{err}");
        bamboo_unknown_error!("user", "Failed to hash the password")
    })?;

    user::Entity::update_many()
        .col_expr(user::Column::Password, Expr::value(hashed_password))
        .col_expr(
            user::Column::TotpSecret,
            Expr::value::<Option<Vec<u8>>>(None),
        )
        .col_expr(user::Column::TotpSecretEncrypted, Expr::value(false))
        .col_expr(user::Column::TotpValidated, Expr::value(false))
        .filter(user::Column::Id.eq(id))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            bamboo_db_error!("user", "Failed to update user")
        })
        .map(|_| ())
}

pub async fn update_me(
    id: i32,
    email: String,
    display_name: String,
    discord_name: String,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    user::Entity::update_many()
        .col_expr(user::Column::Email, Expr::value(email))
        .col_expr(user::Column::DisplayName, Expr::value(display_name))
        .col_expr(user::Column::DiscordName, Expr::value(discord_name))
        .filter(user::Column::Id.eq(id))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            bamboo_db_error!("user", "Failed to update user")
        })
        .map(|_| ())
}

pub async fn change_my_password(
    id: i32,
    old_password: String,
    new_password: String,
    db: &DatabaseConnection,
) -> Result<(), PasswordError> {
    let hashed_password = bcrypt::hash(new_password.clone(), 12).map_err(|err| {
        log::error!("{err}");
        PasswordError::UnknownError
    })?;

    let user = get_user(id, db)
        .await
        .map_err(|_| PasswordError::UserNotFound)?;
    let is_valid = user.validate_password(old_password.clone());

    if !is_valid {
        return Err(PasswordError::WrongPassword);
    }

    let (totp_secret, totp_secret_encrypted) = if user.totp_validated.unwrap_or(false) {
        let decrypted_totp_secret = if user.totp_secret_encrypted {
            decrypt_string(user.totp_secret.clone().unwrap(), old_password)
                .map_err(|_| PasswordError::UnknownError)?
        } else {
            user.totp_secret.clone().unwrap()
        };

        let encrypted_totp_secret = encrypt_string(decrypted_totp_secret, new_password.clone())
            .map_err(|_| PasswordError::UnknownError)?;

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
            PasswordError::UnknownError
        })
        .map(|_| ())
}

pub async fn get_user_by_token(token: String, db: &DatabaseConnection) -> BambooResult<User> {
    match user::Entity::find()
        .filter(token::Column::Token.eq(token))
        .join(JoinType::InnerJoin, user::Relation::Token.def())
        .one(db)
        .await
    {
        Ok(Some(user)) => Ok(user),
        Ok(None) => Err(bamboo_unauthorized_error!(
            "authentication",
            "Token or user not found"
        )),
        Err(err) => {
            log::error!("Failed to get user by token {err}");
            Err(bamboo_unauthorized_error!(
                "authentication",
                "Token or user not found"
            ))
        }
    }
}

pub async fn enable_totp(id: i32, secret: Vec<u8>, db: &DatabaseConnection) -> BambooErrorResult {
    user::Entity::update_many()
        .col_expr(user::Column::TotpSecret, Expr::value(secret))
        .col_expr(user::Column::TotpSecretEncrypted, Expr::value(false))
        .col_expr(user::Column::TotpValidated, Expr::value(false))
        .filter(user::Column::Id.eq(id))
        .exec(db)
        .await
        .map_err(|_| bamboo_db_error!("user", "The secret could not be saved"))
        .map(|_| ())
}

pub async fn disable_totp(id: i32, db: &DatabaseConnection) -> BambooErrorResult {
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
        .map_err(|_| bamboo_db_error!("user", "Failed to disable totp"))
        .map(|_| ())
}

pub async fn validate_totp(
    id: i32,
    password: String,
    code: String,
    db: &DatabaseConnection,
) -> BambooResult<bool> {
    let user = get_user(id, db).await?;
    let valid = validate_login(id, code, password.clone(), true, db)
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
        .map_err(|_| bamboo_db_error!("user", "Totp could not be validated"))
        .map(|_| valid)
}

pub async fn validate_login(
    id: i32,
    code: String,
    password: String,
    initial_validation: bool,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    let user = get_user(id, db).await?;

    let password_valid = user.validate_password(password.clone());
    if !password_valid {
        return Err(bamboo_unauthorized_error!("user", "Invalid login data"));
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
    user: Model,
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
            .map_err(|_| bamboo_db_error!("user", "Failed to validate"))?;

        decrypted_secret
    };

    let is_totp_valid = match totp_rs::TOTP::from_rfc6238(
        totp_rs::Rfc6238::new(
            6,
            totp_secret.clone(),
            Some("Bambushain".to_string()),
            user.display_name.clone(),
        )
        .expect("Should be valid"),
    ) {
        Ok(totp) => totp.check_current(code.as_str()).unwrap_or_else(|err| {
            log::error!("Failed to validate totp {err}");
            false
        }),
        Err(err) => {
            log::error!("Failed to create totp url {err}");
            false
        }
    };

    if is_totp_valid {
        Ok(())
    } else {
        Err(bamboo_crypto_error!("user", "Failed to validate"))
    }
}

fn validate_email_token(code: String, password: String, user: Model) -> BambooErrorResult {
    let two_factor_code = String::from_utf8_lossy(&decrypt_string(
        base64::prelude::BASE64_STANDARD
            .decode(user.two_factor_code.unwrap())
            .map_err(|_| bamboo_unauthorized_error!("user", "Failed to validate"))?,
        password.clone(),
    )?)
    .into_owned();

    if two_factor_code.eq(&code) {
        Ok(())
    } else {
        Err(bamboo_unauthorized_error!("user", "Failed to validate"))
    }
}

use rand::distributions::Uniform;
use rand::Rng;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, NotSet, QueryFilter};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::Expr;

use pandaparty_entities::{pandaparty_db_error, pandaparty_validation_error};
use pandaparty_entities::prelude::*;

pub async fn validate_auth_and_create_token(email: String, password: String, two_factor_code: String, db: &DatabaseConnection) -> PandaPartyResult<LoginResult> {
    let user = match crate::user::get_user_by_email(email.clone(), db).await {
        Ok(user) => user,
        Err(err) => {
            log::error!("Failed to load user {}: {err}", email);
            return Err(pandaparty_not_found_error!("user", "User not found"));
        }
    };

    let password_valid = user.validate_password(password);
    if !password_valid {
        return Err(pandaparty_validation_error!("token", "Password is invalid"));
    }

    let two_factor_code_valid = user.two_factor_code.eq(&two_factor_code);
    if !two_factor_code_valid {
        return Err(pandaparty_validation_error!("token", "Two factor code is invalid"));
    }

    let token = pandaparty_entities::token::ActiveModel {
        id: NotSet,
        token: Set(uuid::Uuid::new_v4().to_string()),
        user_id: Set(user.id),
    };

    match token.insert(db).await {
        Ok(token) => Ok(LoginResult {
            token: token.token,
            user: user.to_web_user(),
        }),
        Err(err) => {
            log::error!("{err}");
            Err(pandaparty_db_error!("token", "Failed to create token"))
        }
    }
}

pub async fn validate_auth_and_set_two_factor_code(email: String, password: String, db: &DatabaseConnection) -> PandaPartyResult<TwoFactorResult> {
    let user = match crate::user::get_user_by_email(email.clone(), db).await {
        Ok(user) => user,
        Err(err) => {
            log::error!("Failed to load user {}: {err}", email);
            return Err(pandaparty_not_found_error!("user", "User not found"));
        }
    };

    let password_valid = user.validate_password(password);
    if !password_valid {
        return Err(pandaparty_validation_error!("token", "Password is invalid"));
    }

    let two_factor_code = rand::thread_rng()
        .sample_iter(&Uniform::new(0, 10))
        .take(6)
        .map( |id| id.to_string())
        .collect::<Vec<String>>()
        .join("");

    if let Err(_) = pandaparty_entities::user::Entity::update_many()
        .col_expr(pandaparty_entities::user::Column::TwoFactorCode, Expr::value(two_factor_code.clone()))
        .filter(pandaparty_entities::user::Column::Id.eq(user.id))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("user", "Failed to update user")
        })
        .map(|_| ()) {
        return Err(pandaparty_validation_error!("token", "Failed to set two factor code"));
    }

    Ok(TwoFactorResult {
        user: user.to_web_user(),
        two_factor_code,
    })
}

pub async fn delete_token(token: String, db: &DatabaseConnection) -> PandaPartyErrorResult {
    pandaparty_entities::token::Entity::delete_many()
        .filter(pandaparty_entities::token::Column::Token.eq(token))
        .exec(db)
        .await
        .map(|_| ())
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("token", "Failed to delete the token")
        })
}

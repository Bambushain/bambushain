use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, NotSet, QueryFilter};
use sea_orm::ActiveValue::Set;

use pandaparty_entities::{pandaparty_db_error, pandaparty_validation_error};
use pandaparty_entities::prelude::*;

pub async fn validate_auth_and_create_token(username: String, password: String, db: &DatabaseConnection) -> PandaPartyResult<LoginResult> {
    let user = match crate::user::get_user_by_username(username.clone(), db).await {
        Ok(user) => user,
        Err(err) => {
            log::error!("Failed to load user {}: {err}", username);
            return Err(pandaparty_not_found_error!("user", "User not found"));
        }
    };
    let is_valid = user.validate_password(password);

    if !is_valid {
        return Err(pandaparty_validation_error!("token", "Password is invalid"));
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

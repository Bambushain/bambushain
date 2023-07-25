use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, NotSet, QueryFilter};
use sea_orm::ActiveValue::Set;

use sheef_entities::{sheef_db_error, sheef_validation_error};
use sheef_entities::prelude::*;

pub async fn validate_auth_and_create_token(username: String, password: String) -> SheefResult<LoginResult> {
    let user = match crate::user::get_user(username.clone()).await {
        Ok(user) => user,
        Err(err) => {
            log::error!("Failed to load user {}: {err}", username);
            return Err(sheef_entities::sheef_not_found_error!("user", "User not found"));
        }
    };
    let is_valid = user.validate_password(password);

    if !is_valid {
        return Err(sheef_validation_error!("token", "Password is invalid"));
    }

    let token = sheef_entities::token::ActiveModel {
        id: NotSet,
        token: Set(uuid::Uuid::new_v4().to_string()),
        user_id: Set(user.id),
    };

    let db = open_db_connection!();

    match token.insert(&db).await {
        Ok(token) => Ok(LoginResult {
            token: token.token,
            user: user.to_web_user(),
        }),
        Err(err) => {
            log::error!("{err}");
            Err(sheef_db_error!("token", "Failed to create token"))
        }
    }
}

pub async fn delete_token(username: String, token: String) -> SheefErrorResult {
    let user = get_user_by_username!(username);
    let db = open_db_connection!();

    sheef_entities::token::Entity::delete_many()
        .filter(sheef_entities::token::Column::Token.eq(token))
        .filter(sheef_entities::token::Column::UserId.eq(user.id))
        .exec(&db)
        .await
        .map(|_| ())
        .map_err(|err| {
            log::error!("{err}");
            sheef_db_error!("token", "Failed to delete the token")
        })
}

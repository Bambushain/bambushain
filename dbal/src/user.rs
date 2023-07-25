use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, JoinType, NotSet, QueryFilter, QueryOrder, QuerySelect, RelationTrait};
use sea_orm::sea_query::Expr;

use sheef_entities::{sheef_db_error, sheef_not_found_error, sheef_unauthorized_error, token, user};
use sheef_entities::prelude::*;

pub async fn get_user(username: String) -> SheefResult<User> {
    let db = open_db_connection!();

    let result = match user::Entity::find().filter(user::Column::Username.eq(username)).one(&db).await {
        Ok(Some(res)) => Ok(res),
        Ok(None) => Err(sheef_not_found_error!("user", "The user was not found")),
        Err(err) => {
            log::error!("{err}");
            Err(sheef_db_error!("user", "Failed to execute database query"))
        }
    };

    let _ = db.close().await;

    result
}

pub async fn get_users() -> SheefResult<Vec<User>> {
    let db = open_db_connection!();

    let result = user::Entity::find()
        .order_by_asc(user::Column::Username)
        .filter(user::Column::IsHidden.eq(false))
        .all(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            sheef_db_error!("user", "Failed to load users")
        });

    let _ = db.close().await;

    result
}

pub async fn user_exists(username: String) -> bool {
    get_user(username).await.is_ok()
}

pub async fn create_user(user: User) -> SheefResult<User> {
    let db = open_db_connection!();

    let mut model = user.into_active_model();
    model.id = NotSet;
    let result = model.insert(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            sheef_db_error!("user", "Failed to create user")
        });

    let _ = db.close().await;

    result
}

pub async fn delete_user(username: String) -> SheefErrorResult {
    let db = open_db_connection!();

    let result = user::Entity::delete_many()
        .filter(user::Column::Username.eq(username))
        .exec(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            sheef_db_error!("user", "Failed to delete user")
        })
        .map(|_| ());

    let _ = db.close().await;

    result
}

pub async fn change_mod_status(username: String, is_mod: bool) -> SheefErrorResult {
    let db = open_db_connection!();

    let result = user::Entity::update_many()
        .col_expr(user::Column::IsMod, Expr::value(is_mod))
        .filter(user::Column::Username.eq(username))
        .exec(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            sheef_db_error!("user", "Failed to update user")
        })
        .map(|_| ());

    let _ = db.close().await;

    result
}

pub async fn change_main_group(username: String, is_main_group: bool) -> SheefErrorResult {
    let db = open_db_connection!();

    let result = user::Entity::update_many()
        .col_expr(user::Column::IsMainGroup, Expr::value(is_main_group))
        .filter(user::Column::Username.eq(username))
        .exec(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            sheef_db_error!("user", "Failed to update user")
        })
        .map(|_| ());

    let _ = db.close().await;

    result
}

pub async fn change_password(username: String, password: String) -> SheefErrorResult {
    let hashed_password = match bcrypt::hash(password, 12) {
        Ok(pw) => pw,
        Err(err) => {
            log::error!("{err}");

            return Err(sheef_unknown_error!("user", "Failed to hash the password"));
        }
    };

    let db = open_db_connection!();

    let result = user::Entity::update_many()
        .col_expr(user::Column::Password, Expr::value(hashed_password))
        .filter(user::Column::Username.eq(username))
        .exec(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            sheef_db_error!("user", "Failed to update user")
        })
        .map(|_| ());

    let _ = db.close().await;

    result
}

pub async fn update_me(username: String, job: String, gear_level: String) -> SheefErrorResult {
    let db = open_db_connection!();
    let result = user::Entity::update_many()
        .col_expr(user::Column::Job, Expr::value(job))
        .col_expr(user::Column::GearLevel, Expr::value(gear_level))
        .filter(user::Column::Username.eq(username))
        .exec(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            sheef_db_error!("user", "Failed to update user")
        })
        .map(|_| ());

    let _ = db.close().await;

    result
}

pub async fn change_my_password(username: String, old_password: String, new_password: String) -> Result<(), PasswordError> {
    let hashed_password = match bcrypt::hash(new_password, 12) {
        Ok(pw) => pw,
        Err(_) => return Err(PasswordError::UnknownError)
    };

    let user = match get_user(username.clone()).await {
        Ok(user) => user,
        Err(_) => return Err(PasswordError::UserNotFound)
    };
    let is_valid = user.validate_password(old_password.clone());

    if !is_valid {
        return Err(PasswordError::WrongPassword);
    }

    let db = match open_db_connection_with_error!() {
        Ok(db) => db,
        Err(err) => {
            log::error!("Failed to connect to database {err}");
            return Err(PasswordError::UnknownError);
        }
    };

    let result = user::Entity::update_many()
        .col_expr(user::Column::Password, Expr::value(hashed_password))
        .filter(user::Column::Username.eq(username))
        .exec(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            PasswordError::UnknownError
        })
        .map(|_| ());

    let _ = db.close().await;

    result
}

pub async fn get_user_by_token(token: String) -> SheefResult<User> {
    let db = open_db_connection!();

    let result = match user::Entity::find()
        .filter(token::Column::Token.eq(token))
        .join(JoinType::InnerJoin, user::Relation::Token.def())
        .one(&db)
        .await {
        Ok(Some(user)) => Ok(user),
        Ok(None) => Err(sheef_unauthorized_error!("authentication", "Token or user not found")),
        Err(err) => {
            log::error!("Failed to get user by token {err}");
            Err(sheef_unauthorized_error!("authentication", "Token or user not found"))
        }
    };

    let _ = db.close().await;

    result
}
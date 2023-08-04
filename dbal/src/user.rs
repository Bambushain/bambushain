use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, JoinType, NotSet, QueryFilter, QueryOrder, QuerySelect, RelationTrait};
use sea_orm::sea_query::Expr;

use pandaparty_entities::{pandaparty_db_error, pandaparty_not_found_error, pandaparty_unauthorized_error, token, user};
use pandaparty_entities::prelude::*;

pub async fn get_user(username: String, db: &DatabaseConnection) -> SheefResult<User> {
    match user::Entity::find()
        .filter(user::Column::Username.eq(username))
        .one(db)
        .await {
        Ok(Some(res)) => Ok(res),
        Ok(None) => Err(pandaparty_not_found_error!("user", "The user was not found")),
        Err(err) => {
            log::error!("{err}");
            Err(pandaparty_db_error!("user", "Failed to execute database query"))
        }
    }
}

pub async fn get_users(db: &DatabaseConnection) -> SheefResult<Vec<User>> {
    user::Entity::find()
        .order_by_asc(user::Column::Username)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("user", "Failed to load users")
        })
}

pub async fn user_exists(username: String, db: &DatabaseConnection) -> bool {
    get_user(username, db).await.is_ok()
}

pub async fn create_user(user: User, db: &DatabaseConnection) -> SheefResult<User> {
    let mut model = user.into_active_model();
    model.id = NotSet;
    let _ = model.set_password(model.clone().password.as_ref());

    model.insert(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("user", "Failed to create user")
        })
}

pub async fn delete_user(username: String, db: &DatabaseConnection) -> SheefErrorResult {
    user::Entity::delete_many()
        .filter(user::Column::Username.eq(username))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("user", "Failed to delete user")
        })
        .map(|_| ())
}

pub async fn change_mod_status(username: String, is_mod: bool, db: &DatabaseConnection) -> SheefErrorResult {
    user::Entity::update_many()
        .col_expr(user::Column::IsMod, Expr::value(is_mod))
        .filter(user::Column::Username.eq(username))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("user", "Failed to update user")
        })
        .map(|_| ())
}

pub async fn change_password(username: String, password: String, db: &DatabaseConnection) -> SheefErrorResult {
    let hashed_password = match bcrypt::hash(password, 12) {
        Ok(pw) => pw,
        Err(err) => {
            log::error!("{err}");

            return Err(pandaparty_unknown_error!("user", "Failed to hash the password"));
        }
    };

    user::Entity::update_many()
        .col_expr(user::Column::Password, Expr::value(hashed_password))
        .filter(user::Column::Username.eq(username))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("user", "Failed to update user")
        })
        .map(|_| ())
}

pub async fn update_me(username: String, job: String, gear_level: String, db: &DatabaseConnection) -> SheefErrorResult {
    user::Entity::update_many()
        .col_expr(user::Column::Job, Expr::value(job))
        .col_expr(user::Column::GearLevel, Expr::value(gear_level))
        .filter(user::Column::Username.eq(username))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("user", "Failed to update user")
        })
        .map(|_| ())
}

pub async fn change_my_password(username: String, old_password: String, new_password: String, db: &DatabaseConnection) -> Result<(), PasswordError> {
    let hashed_password = match bcrypt::hash(new_password, 12) {
        Ok(pw) => pw,
        Err(_) => return Err(PasswordError::UnknownError)
    };

    let user = match get_user(username.clone(), db).await {
        Ok(user) => user,
        Err(_) => return Err(PasswordError::UserNotFound)
    };
    let is_valid = user.validate_password(old_password.clone());

    if !is_valid {
        return Err(PasswordError::WrongPassword);
    }

    user::Entity::update_many()
        .col_expr(user::Column::Password, Expr::value(hashed_password))
        .filter(user::Column::Username.eq(username))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            PasswordError::UnknownError
        })
        .map(|_| ())
}

pub async fn get_user_by_token(token: String, db: &DatabaseConnection) -> SheefResult<User> {
    match user::Entity::find()
        .filter(token::Column::Token.eq(token))
        .join(JoinType::InnerJoin, user::Relation::Token.def())
        .one(db)
        .await {
        Ok(Some(user)) => Ok(user),
        Ok(None) => Err(pandaparty_unauthorized_error!("authentication", "Token or user not found")),
        Err(err) => {
            log::error!("Failed to get user by token {err}");
            Err(pandaparty_unauthorized_error!("authentication", "Token or user not found"))
        }
    }
}
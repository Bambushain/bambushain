use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, JoinType, NotSet, QueryFilter, QueryOrder, QuerySelect, RelationTrait};
use sea_orm::prelude::*;
use sea_orm::sea_query::Expr;

use pandaparty_entities::{pandaparty_db_error, pandaparty_not_found_error, pandaparty_unauthorized_error, token, user};
use pandaparty_entities::prelude::*;

pub async fn get_user(id: i32, db: &DatabaseConnection) -> PandaPartyResult<User> {
    match user::Entity::find_by_id(id)
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

pub async fn get_user_by_email(email: String, db: &DatabaseConnection) -> PandaPartyResult<User> {
    match user::Entity::find()
        .filter(user::Column::Email.eq(email))
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

pub async fn get_users(db: &DatabaseConnection) -> PandaPartyResult<Vec<User>> {
    user::Entity::find()
        .order_by_asc(user::Column::Email)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("user", "Failed to load users")
        })
}

pub async fn user_exists(id: i32, db: &DatabaseConnection) -> bool {
    match user::Entity::find_by_id(id)
        .select_only()
        .column(user::Column::Id)
        .count(db)
        .await {
        Ok(count) => count > 0,
        _ => false
    }
}

pub async fn create_user(user: User, db: &DatabaseConnection) -> PandaPartyResult<User> {
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

pub async fn delete_user(id: i32, db: &DatabaseConnection) -> PandaPartyErrorResult {
    user::Entity::delete_by_id(id)
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("user", "Failed to delete user")
        })
        .map(|_| ())
}

pub async fn change_mod_status(id: i32, is_mod: bool, db: &DatabaseConnection) -> PandaPartyErrorResult {
    user::Entity::update_many()
        .filter(user::Column::Id.eq(id))
        .col_expr(user::Column::IsMod, Expr::value(is_mod))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("user", "Failed to update user")
        })
        .map(|_| ())
}

pub async fn change_password(id: i32, password: String, db: &DatabaseConnection) -> PandaPartyErrorResult {
    let hashed_password = match bcrypt::hash(password, 12) {
        Ok(pw) => pw,
        Err(err) => {
            log::error!("{err}");

            return Err(pandaparty_unknown_error!("user", "Failed to hash the password"));
        }
    };

    user::Entity::update_many()
        .col_expr(user::Column::Password, Expr::value(hashed_password))
        .filter(user::Column::Id.eq(id))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("user", "Failed to update user")
        })
        .map(|_| ())
}

pub async fn update_me(id: i32, email: String, display_name: String, discord_name: String, db: &DatabaseConnection) -> PandaPartyErrorResult {
    user::Entity::update_many()
        .col_expr(user::Column::Email, Expr::value(email))
        .col_expr(user::Column::DisplayName, Expr::value(display_name))
        .col_expr(user::Column::DiscordName, Expr::value(discord_name))
        .filter(user::Column::Id.eq(id))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("user", "Failed to update user")
        })
        .map(|_| ())
}

pub async fn change_my_password(id: i32, old_password: String, new_password: String, db: &DatabaseConnection) -> Result<(), PasswordError> {
    let hashed_password = match bcrypt::hash(new_password, 12) {
        Ok(pw) => pw,
        Err(_) => return Err(PasswordError::UnknownError)
    };

    let user = match get_user(id, db).await {
        Ok(user) => user,
        Err(_) => return Err(PasswordError::UserNotFound)
    };
    let is_valid = user.validate_password(old_password.clone());

    if !is_valid {
        return Err(PasswordError::WrongPassword);
    }

    user::Entity::update_many()
        .col_expr(user::Column::Password, Expr::value(hashed_password))
        .filter(user::Column::Id.eq(id))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            PasswordError::UnknownError
        })
        .map(|_| ())
}

pub async fn get_user_by_token(token: String, db: &DatabaseConnection) -> PandaPartyResult<User> {
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
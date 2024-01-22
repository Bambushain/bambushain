use sea_orm::prelude::*;
use sea_orm::sea_query::Expr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel,
    JoinType, NotSet, QueryFilter, QueryOrder, QuerySelect, RelationTrait, Set,
};

use bamboo_common_core::entities::*;
use bamboo_common_core::error::*;

use crate::prelude::dbal;

pub async fn get_user_by_id_only(id: i32, db: &DatabaseConnection) -> BambooResult<User> {
    user::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("user", "Failed to execute database query")
        })
        .map(|data| {
            if let Some(data) = data {
                Ok(data)
            } else {
                Err(BambooError::not_found("user", "The user was not found"))
            }
        })?
}

pub async fn get_user(grove_id: i32, id: i32, db: &DatabaseConnection) -> BambooResult<User> {
    user::Entity::find_by_id(id)
        .filter(user::Column::GroveId.eq(grove_id))
        .one(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("user", "Failed to execute database query")
        })
        .map(|data| {
            if let Some(data) = data {
                Ok(data)
            } else {
                Err(BambooError::not_found("user", "The user was not found"))
            }
        })?
}

pub async fn get_user_by_token(token: String, db: &DatabaseConnection) -> BambooResult<User> {
    user::Entity::find()
        .filter(token::Column::Token.eq(token))
        .join(JoinType::InnerJoin, user::Relation::Token.def())
        .one(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::unauthorized("authentication", "Token or user not found")
        })
        .map(|data| {
            if let Some(data) = data {
                Ok(data)
            } else {
                Err(BambooError::unauthorized(
                    "authentication",
                    "Token or user not found",
                ))
            }
        })?
}

pub async fn get_user_by_email_or_username(
    username: String,
    db: &DatabaseConnection,
) -> BambooResult<User> {
    user::Entity::find()
        .filter(
            Condition::all()
                .add(
                    Condition::any()
                        .add(user::Column::Email.eq(username.clone()))
                        .add(user::Column::DisplayName.eq(username)),
                )
                .add(
                    Condition::any()
                        .add(
                            Condition::all()
                                .add(grove::Column::IsSuspended.eq(false))
                                .add(grove::Column::IsEnabled.eq(true)),
                        )
                        .add(
                            Condition::all()
                                .add(grove::Column::IsSuspended.eq(false))
                                .add(grove::Column::IsEnabled.eq(false))
                                .add(user::Column::IsMod.eq(true)),
                        ),
                ),
        )
        .inner_join(grove::Entity)
        .one(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("user", "Failed to execute database query")
        })
        .map(|data| {
            if let Some(data) = data {
                Ok(data)
            } else {
                Err(BambooError::not_found("user", "The user was not found"))
            }
        })?
}

pub async fn get_users(grove_id: i32, db: &DatabaseConnection) -> BambooResult<Vec<User>> {
    user::Entity::find()
        .filter(user::Column::GroveId.eq(grove_id))
        .order_by_asc(user::Column::Email)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("user", "Failed to load users")
        })
}

pub async fn get_users_with_mod_rights(
    user_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<Vec<User>> {
    let grove = dbal::get_grove_by_user_id(user_id, db).await?;

    user::Entity::find()
        .filter(user::Column::IsMod.eq(true))
        .filter(user::Column::GroveId.eq(grove.id))
        .order_by_asc(user::Column::Email)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("user", "Failed to load users")
        })
}

pub(crate) async fn user_exists_by_id(
    id: i32,
    email: String,
    name: String,
    db: &DatabaseConnection,
) -> BambooResult<bool> {
    user::Entity::find()
        .filter(user::Column::Id.ne(id))
        .filter(
            Condition::any()
                .add(user::Column::Email.eq(email))
                .add(user::Column::DisplayName.eq(name)),
        )
        .count(db)
        .await
        .map(|count| count > 0)
        .map_err(|err| {
            log::error!("Failed to load users {err}");
            BambooError::database("user", "Failed to load users")
        })
}

async fn user_exists_by_email_and_name(
    email: String,
    name: String,
    db: &DatabaseConnection,
) -> BambooResult<bool> {
    user::Entity::find()
        .filter(
            Condition::any()
                .add(user::Column::Email.eq(email))
                .add(user::Column::DisplayName.eq(name)),
        )
        .count(db)
        .await
        .map(|count| count > 0)
        .map_err(|err| {
            log::error!("Failed to load users {err}");
            BambooError::database("user", "Failed to load users")
        })
}

pub async fn create_user(
    grove_id: i32,
    user: User,
    password: String,
    db: &DatabaseConnection,
) -> BambooResult<User> {
    if user_exists_by_email_and_name(user.email.clone(), user.display_name.clone(), db).await? {
        return Err(BambooError::exists_already(
            "user",
            "A user with that email or name exists already",
        ));
    }

    let mut model = user.into_active_model();
    model.id = NotSet;
    model.grove_id = Set(grove_id);
    model.set_password(&password).map_err(|err| {
        log::error!("{err}");
        BambooError::database("user", "Failed to hash password user")
    })?;

    model.insert(db).await.map_err(|err| {
        log::error!("{err}");
        BambooError::database("user", "Failed to create user")
    })
}

pub async fn delete_user(grove_id: i32, id: i32, db: &DatabaseConnection) -> BambooErrorResult {
    user::Entity::delete_by_id(id)
        .filter(user::Column::GroveId.eq(grove_id))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("user", "Failed to delete user")
        })
        .map(|_| ())
}

pub async fn change_mod_status(
    grove_id: i32,
    id: i32,
    is_mod: bool,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    user::Entity::update_many()
        .filter(user::Column::GroveId.eq(grove_id))
        .filter(user::Column::Id.eq(id))
        .col_expr(user::Column::IsMod, Expr::value(is_mod))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("user", "Failed to update user")
        })
        .map(|_| ())
}

pub async fn change_password(
    grove_id: i32,
    id: i32,
    password: String,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    let hashed_password = bcrypt::hash(password, 12).map_err(|err| {
        log::error!("{err}");
        BambooError::unknown("user", "Failed to hash the password")
    })?;

    token::Entity::delete_many()
        .filter(token::Column::UserId.eq(id))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("user", "Failed to update user")
        })
        .map(|_| ())?;

    user::Entity::update_many()
        .col_expr(user::Column::Password, Expr::value(hashed_password))
        .col_expr(
            user::Column::TotpSecret,
            Expr::value::<Option<Vec<u8>>>(None),
        )
        .col_expr(user::Column::TotpSecretEncrypted, Expr::value(false))
        .col_expr(user::Column::TotpValidated, Expr::value(false))
        .filter(user::Column::GroveId.eq(grove_id))
        .filter(user::Column::Id.eq(id))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("user", "Failed to update user")
        })
        .map(|_| ())
}

pub async fn update_profile(
    grove_id: i32,
    id: i32,
    email: String,
    display_name: String,
    discord_name: String,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    if user_exists_by_id(id, email.clone(), display_name.clone(), db).await? {
        return Err(BambooError::exists_already(
            "user",
            "A user with that email or name exists already",
        ));
    }

    user::Entity::update_many()
        .col_expr(user::Column::Email, Expr::value(email))
        .col_expr(user::Column::DisplayName, Expr::value(display_name))
        .col_expr(user::Column::DiscordName, Expr::value(discord_name))
        .filter(user::Column::GroveId.eq(grove_id))
        .filter(user::Column::Id.eq(id))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("user", "Failed to update user")
        })
        .map(|_| ())
}

pub async fn disable_totp(grove_id: i32, id: i32, db: &DatabaseConnection) -> BambooErrorResult {
    user::Entity::update_many()
        .col_expr(
            user::Column::TotpSecret,
            Expr::value::<Option<Vec<u8>>>(None),
        )
        .col_expr(user::Column::TotpValidated, Expr::value(false))
        .col_expr(user::Column::TotpSecretEncrypted, Expr::value(false))
        .filter(user::Column::Id.eq(id))
        .filter(user::Column::GroveId.eq(grove_id))
        .exec(db)
        .await
        .map_err(|_| BambooError::database("user", "Failed to disable totp"))
        .map(|_| ())
}

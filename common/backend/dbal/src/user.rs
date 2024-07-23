use sea_orm::prelude::*;
use sea_orm::sea_query::{Alias, Expr, IntoIden, Query, TableRef};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel,
    JoinType, NotSet, QueryFilter, QueryOrder, QuerySelect, RelationTrait,
};

use bamboo_common_core::entities::user::WebUser;
use bamboo_common_core::entities::*;
use bamboo_common_core::error::*;

pub async fn get_user(id: i32, db: &DatabaseConnection) -> BambooResult<User> {
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
            Condition::any()
                .add(user::Column::Email.eq(username.clone()))
                .add(user::Column::DisplayName.eq(username)),
        )
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

pub async fn get_users(user_id: i32, db: &DatabaseConnection) -> BambooResult<Vec<WebUser>> {
    user::Entity::find()
        .distinct()
        .join_rev(
            JoinType::InnerJoin,
            grove_user::Entity::belongs_to(user::Entity)
                .from(grove_user::Column::UserId)
                .to(user::Column::Id)
                .into(),
        )
        .filter(
            grove_user::Column::GroveId.in_subquery(
                Query::select()
                    .column(grove_user::Column::GroveId)
                    .from(TableRef::SchemaTable(
                        Alias::new("grove").into_iden(),
                        Alias::new("grove_user").into_iden(),
                    ))
                    .cond_where(grove_user::Column::UserId.eq(user_id))
                    .to_owned(),
            ),
        )
        .order_by_asc(user::Column::DisplayName)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("user", "Failed to load users")
        })
        .map(|users| {
            users
                .iter()
                .cloned()
                .map(|user| WebUser::from_user(user))
                .collect::<Vec<WebUser>>()
        })
}

pub async fn get_users_by_grove(
    user_id: i32,
    grove_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<Vec<user::GroveUser>> {
    let sub_query = &mut grove_user::Entity::find()
        .select_only()
        .column(grove_user::Column::GroveId)
        .filter(grove_user::Column::UserId.eq(user_id));

    user::Entity::find()
        .select_only()
        .column_as(user::Column::Id, "id")
        .column_as(user::Column::Email, "email")
        .column_as(user::Column::DiscordName, "discord_name")
        .column_as(user::Column::DisplayName, "display_name")
        .column_as(grove_user::Column::IsMod, "is_mod")
        .join(
            JoinType::LeftJoin,
            user::Entity::belongs_to(grove_user::Entity)
                .from(user::Column::Id)
                .to(grove_user::Column::UserId)
                .into(),
        )
        .filter(grove_user::Column::GroveId.eq(grove_id))
        .filter(grove_user::Column::GroveId.in_subquery(QuerySelect::query(sub_query).to_owned()))
        .order_by_asc(user::Column::DisplayName)
        .into_model::<user::GroveUser>()
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
    model.set_password(&password).map_err(|err| {
        log::error!("{err}");
        BambooError::database("user", "Failed to hash password user")
    })?;

    model.insert(db).await.map_err(|err| {
        log::error!("{err}");
        BambooError::database("user", "Failed to create user")
    })
}

pub async fn delete_user(id: i32, db: &DatabaseConnection) -> BambooErrorResult {
    user::Entity::delete_by_id(id)
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("user", "Failed to delete user")
        })
        .map(|_| ())
}

pub async fn set_password(id: i32, password: String, db: &DatabaseConnection) -> BambooErrorResult {
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
        .filter(user::Column::Id.eq(id))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("user", "Failed to update user")
        })
        .map(|_| ())
}

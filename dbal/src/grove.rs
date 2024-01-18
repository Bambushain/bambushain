use sea_orm::prelude::*;
use sea_orm::{IntoActiveModel, NotSet};

use bamboo_entities::prelude::*;
use bamboo_error::*;

use crate::authentication::get_tokens_by_grove;

pub async fn get_grove_by_user_id(user_id: i32, db: &DatabaseConnection) -> BambooResult<Grove> {
    grove::Entity::find()
        .filter(user::Column::Id.eq(user_id))
        .inner_join(user::Entity)
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

pub async fn get_grove_by_id(id: i32, db: &DatabaseConnection) -> BambooResult<Grove> {
    grove::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("grove", "Failed to execute database query")
        })
        .map(|data| {
            if let Some(data) = data {
                Ok(data)
            } else {
                Err(BambooError::not_found("grove", "The grove was not found"))
            }
        })?
}

pub async fn get_grove_by_name(name: String, db: &DatabaseConnection) -> BambooResult<Grove> {
    grove::Entity::find()
        .filter(grove::Column::Name.eq(name))
        .one(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("grove", "Failed to execute database query")
        })
        .map(|data| {
            if let Some(data) = data {
                Ok(data)
            } else {
                Err(BambooError::not_found("grove", "The grove was not found"))
            }
        })?
}

pub async fn get_groves(db: &DatabaseConnection) -> BambooResult<Vec<Grove>> {
    grove::Entity::find().all(db).await.map_err(|err| {
        log::error!("{err}");
        BambooError::database("user", "Failed to execute database query")
    })
}

pub async fn create_grove(name: String, db: &DatabaseConnection) -> BambooResult<Grove> {
    let mut active_model = Grove::new(name, false, true).into_active_model();
    active_model.id = NotSet;

    active_model.insert(db).await.map_err(|err| {
        log::error!("Failed to create grove {err}");
        BambooError::database("grove", "Failed to create grove")
    })
}

pub async fn migrate_between_groves(
    old_grove_id: Option<i32>,
    new_grove_id: i32,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    if let Some(id) = old_grove_id {
        user::Entity::update_many().filter(user::Column::GroveId.eq(id))
    } else {
        user::Entity::update_many().filter(user::Column::GroveId.is_null())
    }
    .col_expr(user::Column::GroveId, Expr::value(new_grove_id))
    .exec(db)
    .await
    .map_err(|err| {
        log::error!(
            "Failed to migrate users from grove {old_grove_id:?} to {new_grove_id} grove {err}"
        );
        BambooError::database("grove", "Failed to create grove")
    })
    .map(|_| ())?;

    if let Some(id) = old_grove_id {
        event::Entity::update_many().filter(event::Column::GroveId.eq(id))
    } else {
        event::Entity::update_many().filter(event::Column::GroveId.is_null())
    }
    .col_expr(event::Column::GroveId, Expr::value(new_grove_id))
    .exec(db)
    .await
    .map_err(|err| {
        log::error!(
            "Failed to migrate events from grove {old_grove_id:?} to {new_grove_id} grove {err}"
        );
        BambooError::database("grove", "Failed to create grove")
    })
    .map(|_| ())
}

pub async fn disable_grove(id: i32, db: &DatabaseConnection) -> BambooErrorResult {
    let tokens = get_tokens_by_grove(id, db).await?;
    let tokens = tokens.iter().map(|token| token.id);
    let _ = grove::Entity::delete_many()
        .filter(token::Column::Id.is_in(tokens))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("Failed to delete tokens, disable grove anyway, {err}");
        });

    grove::Entity::update_many()
        .filter(grove::Column::Id.eq(id))
        .col_expr(grove::Column::IsEnabled, Expr::value(false))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("grove", "Failed to disable grove")
        })
        .map(|_| ())
}

pub async fn enable_grove(id: i32, db: &DatabaseConnection) -> BambooErrorResult {
    grove::Entity::update_many()
        .filter(grove::Column::Id.eq(id))
        .col_expr(grove::Column::IsEnabled, Expr::value(true))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("grove", "Failed to enable grove")
        })
        .map(|_| ())
}

pub async fn delete_grove(id: i32, db: &DatabaseConnection) -> BambooErrorResult {
    grove::Entity::delete_by_id(id)
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("grove", "Failed to delete grove")
        })
        .map(|_| ())
}

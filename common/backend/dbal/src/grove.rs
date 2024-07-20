use sea_orm::prelude::*;
use sea_orm::{IntoActiveModel, NotSet, QueryOrder};

use bamboo_common_core::entities::*;
use bamboo_common_core::error::*;

pub async fn get_groves_by_user(user_id: i32, db: &DatabaseConnection) -> BambooResult<Vec<Grove>> {
    grove::Entity::find()
        .filter(user::Column::Id.eq(user_id))
        .inner_join(grove_user::Entity)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("user", "Failed to execute database query")
        })
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
    grove::Entity::find()
        .order_by_asc(grove::Column::Id)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("user", "Failed to execute database query")
        })
}

pub async fn create_grove(name: String, db: &DatabaseConnection) -> BambooResult<Grove> {
    let mut active_model = Grove::new(name).into_active_model();
    active_model.id = NotSet;

    active_model.insert(db).await.map_err(|err| {
        log::error!("Failed to create grove {err}");
        BambooError::database("grove", "Failed to create grove")
    })
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

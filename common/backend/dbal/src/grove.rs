use bamboo_common_core::entities::*;
use bamboo_common_core::error::*;
use sea_orm::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::{IntoActiveModel, NotSet, QueryOrder};

pub async fn get_grove(id: i32, user_id: i32, db: &DatabaseConnection) -> BambooResult<Grove> {
    grove::Entity::find_by_id(id)
        .inner_join(grove_user::Entity)
        .filter(grove_user::Column::UserId.eq(user_id))
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

pub async fn grove_exists_by_name(name: String, db: &DatabaseConnection) -> BambooResult<bool> {
    grove::Entity::find()
        .filter(grove::Column::Name.eq(name))
        .count(db)
        .await
        .map(|count| count > 0)
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("grove", "Failed to execute database query")
        })
}

pub async fn grove_exists_by_id(
    id: i32,
    name: String,
    db: &DatabaseConnection,
) -> BambooResult<bool> {
    grove::Entity::find()
        .filter(grove::Column::Id.ne(id))
        .filter(grove::Column::Name.eq(name))
        .count(db)
        .await
        .map(|count| count > 0)
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("grove", "Failed to execute database query")
        })
}

pub async fn get_groves(user_id: i32, db: &DatabaseConnection) -> BambooResult<Vec<Grove>> {
    grove::Entity::find()
        .inner_join(grove_user::Entity)
        .filter(grove_user::Column::UserId.eq(user_id))
        .order_by_asc(grove::Column::Id)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("user", "Failed to execute database query")
        })
}

pub async fn get_all_groves(db: &DatabaseConnection) -> BambooResult<Vec<Grove>> {
    grove::Entity::find()
        .order_by_asc(grove::Column::Id)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("user", "Failed to execute database query")
        })
}

pub async fn create_grove(
    name: String,
    user_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<Grove> {
    if grove_exists_by_name(name.clone(), db).await? {
        return Err(BambooError::exists_already(
            "grove",
            "Grove with that name exists already",
        ));
    }

    let mut active_model = Grove::new(name).into_active_model();
    active_model.id = NotSet;

    let grove = active_model.insert(db).await.map_err(|err| {
        log::error!("Failed to create grove {err}");
        BambooError::database("grove", "Failed to create grove")
    })?;

    let mut user = GroveUser::default().into_active_model();
    user.user_id = Set(user_id);
    user.grove_id = Set(grove.id);
    user.is_mod = Set(true);

    user.insert(db).await.map_err(|err| {
        log::error!("Failed to create grove user {err}");
        BambooError::database("grove", "Failed to create grove user")
    })?;

    Ok(grove)
}

pub async fn update_grove(
    id: i32,
    user_id: i32,
    name: String,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    if grove_exists_by_id(id, name.clone(), db).await? {
        return Err(BambooError::exists_already(
            "grove",
            "Grove with that name exists already",
        ));
    }

    let mut grove = get_grove(id, user_id, db).await?.into_active_model();
    grove.name = Set(name);
    grove
        .update(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("grove", "Failed to update grove")
        })
        .map(|_| ())
}

pub async fn delete_grove(id: i32, user_id: i32, db: &DatabaseConnection) -> BambooErrorResult {
    get_grove(id, user_id, db)
        .await?
        .delete(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("grove", "Failed to delete grove")
        })
        .map(|_| ())
}

use sea_orm::{IntoActiveModel, NotSet, QueryOrder};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::*;

use bamboo_common_core::entities::{character, fighter};
use bamboo_common_core::entities::*;
use bamboo_common_core::error::*;

pub async fn get_fighters(
    user_id: i32,
    character_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<Vec<Fighter>> {
    fighter::Entity::find()
        .filter(fighter::Column::CharacterId.eq(character_id))
        .filter(character::Column::UserId.eq(user_id))
        .inner_join(character::Entity)
        .order_by_asc(fighter::Column::Job)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("fighter", "Failed to load fighters")
        })
}

pub async fn get_fighter(
    id: i32,
    user_id: i32,
    character_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<Fighter> {
    fighter::Entity::find_by_id(id)
        .filter(fighter::Column::CharacterId.eq(character_id))
        .filter(character::Column::UserId.eq(user_id))
        .inner_join(character::Entity)
        .one(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("fighter", "Failed to load fighter")
        })
        .map(|res| {
            if let Some(res) = res {
                Ok(res)
            } else {
                Err(BambooError::not_found(
                    "fighter",
                    "The fighter was not found",
                ))
            }
        })?
}

async fn fighter_exists_by_id(
    id: i32,
    user_id: i32,
    character_id: i32,
    job: FighterJob,
    db: &DatabaseConnection,
) -> BambooResult<bool> {
    fighter::Entity::find()
        .filter(fighter::Column::Id.ne(id))
        .filter(fighter::Column::Job.eq(job))
        .filter(fighter::Column::CharacterId.eq(character_id))
        .filter(character::Column::UserId.eq(user_id))
        .inner_join(character::Entity)
        .count(db)
        .await
        .map(|count| count > 0)
        .map_err(|err| {
            log::error!("Failed to load fighter {err}");
            BambooError::database("fighter", "Failed to load the fighters")
        })
}

async fn fighter_exists_by_job(
    user_id: i32,
    character_id: i32,
    job: FighterJob,
    db: &DatabaseConnection,
) -> BambooResult<bool> {
    fighter::Entity::find()
        .filter(fighter::Column::Job.eq(job))
        .filter(fighter::Column::CharacterId.eq(character_id))
        .filter(character::Column::UserId.eq(user_id))
        .inner_join(character::Entity)
        .count(db)
        .await
        .map(|count| count > 0)
        .map_err(|err| {
            log::error!("Failed to load fighter {err}");
            BambooError::database("fighter", "Failed to load the fighters")
        })
}

pub async fn create_fighter(
    user_id: i32,
    character_id: i32,
    fighter: Fighter,
    db: &DatabaseConnection,
) -> BambooResult<Fighter> {
    if fighter_exists_by_job(user_id, character_id, fighter.job, db).await? {
        return Err(BambooError::exists_already(
            "fighter",
            "A fighter with that job exists already",
        ));
    }

    let mut model = fighter.into_active_model();
    model.character_id = Set(character_id);
    model.id = NotSet;

    model.insert(db).await.map_err(|err| {
        log::error!("{err}");
        BambooError::database("fighter", "Failed to create fighter")
    })
}

pub async fn update_fighter(
    id: i32,
    user_id: i32,
    character_id: i32,
    fighter: Fighter,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    if fighter_exists_by_id(id, user_id, character_id, fighter.job, db).await? {
        return Err(BambooError::exists_already(
            "fighter",
            "A fighter with that job exists already",
        ));
    }

    let mut active_fighter = get_fighter(id, user_id, character_id, db)
        .await?
        .into_active_model();
    active_fighter.level = Set(fighter.level);
    active_fighter.gear_score = Set(fighter.gear_score);

    active_fighter
        .update(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("fighter", "Failed to update fighter")
        })
        .map(|_| ())
}

pub async fn delete_fighter(
    id: i32,
    user_id: i32,
    character_id: i32,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    get_fighter(id, user_id, character_id, db)
        .await?
        .delete(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("fighter", "Failed to delete fighter")
        })
        .map(|_| ())
}

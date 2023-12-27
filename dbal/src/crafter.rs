use sea_orm::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::{IntoActiveModel, NotSet, QueryOrder};

use bamboo_entities::prelude::*;
use bamboo_entities::{character, crafter};
use bamboo_error::*;

pub async fn get_crafters(
    user_id: i32,
    character_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<Vec<Crafter>> {
    crafter::Entity::find()
        .filter(crafter::Column::CharacterId.eq(character_id))
        .filter(character::Column::UserId.eq(user_id))
        .inner_join(character::Entity)
        .order_by_asc(crafter::Column::Job)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("crafter", "Failed to load crafters")
        })
}

pub async fn get_crafter(
    id: i32,
    user_id: i32,
    character_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<Crafter> {
    crafter::Entity::find_by_id(id)
        .filter(crafter::Column::CharacterId.eq(character_id))
        .filter(character::Column::UserId.eq(user_id))
        .inner_join(character::Entity)
        .one(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("crafter", "Failed to load crafter")
        })
        .map(|res| {
            if let Some(res) = res {
                Ok(res)
            } else {
                Err(BambooError::not_found(
                    "crafter",
                    "The crafter was not found",
                ))
            }
        })?
}

async fn crafter_exists_by_id(
    id: i32,
    user_id: i32,
    character_id: i32,
    job: CrafterJob,
    db: &DatabaseConnection,
) -> BambooResult<bool> {
    crafter::Entity::find()
        .filter(crafter::Column::Id.ne(id))
        .filter(crafter::Column::Job.eq(job))
        .filter(crafter::Column::CharacterId.eq(character_id))
        .filter(character::Column::UserId.eq(user_id))
        .inner_join(character::Entity)
        .count(db)
        .await
        .map(|count| count > 0)
        .map_err(|err| {
            log::error!("Failed to load crafter {err}");
            BambooError::database("crafter", "Failed to load the crafters")
        })
}

async fn crafter_exists_by_job(
    user_id: i32,
    character_id: i32,
    job: CrafterJob,
    db: &DatabaseConnection,
) -> BambooResult<bool> {
    crafter::Entity::find()
        .filter(crafter::Column::Job.eq(job))
        .filter(crafter::Column::CharacterId.eq(character_id))
        .filter(character::Column::UserId.eq(user_id))
        .inner_join(character::Entity)
        .count(db)
        .await
        .map(|count| count > 0)
        .map_err(|err| {
            log::error!("Failed to load crafter {err}");
            BambooError::database("crafter", "Failed to load the crafters")
        })
}

pub async fn create_crafter(
    user_id: i32,
    character_id: i32,
    crafter: Crafter,
    db: &DatabaseConnection,
) -> BambooResult<Crafter> {
    if crafter_exists_by_job(user_id, character_id, crafter.job, db).await? {
        return Err(BambooError::exists_already(
            "crafter",
            "A crafter with that job exists already",
        ));
    }

    let mut model = crafter.into_active_model();
    model.character_id = Set(character_id);
    model.id = NotSet;

    model.insert(db).await.map_err(|err| {
        log::error!("{err}");
        BambooError::database("crafter", "Failed to create crafter")
    })
}

pub async fn update_crafter(
    id: i32,
    user_id: i32,
    character_id: i32,
    crafter: Crafter,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    if crafter_exists_by_id(id, user_id, character_id, crafter.job, db).await? {
        return Err(BambooError::exists_already(
            "crafter",
            "A crafter with that job exists already",
        ));
    }

    let mut active_crafter = get_crafter(id, user_id, character_id, db)
        .await?
        .into_active_model();
    active_crafter.level = Set(crafter.level);

    active_crafter
        .update(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("crafter", "Failed to update crafter")
        })
        .map(|_| ())
}

pub async fn delete_crafter(
    id: i32,
    user_id: i32,
    character_id: i32,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    get_crafter(id, user_id, character_id, db)
        .await?
        .delete(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("crafter", "Failed to delete crafter")
        })
        .map(|_| ())
}

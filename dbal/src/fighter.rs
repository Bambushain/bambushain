use sea_orm::prelude::*;
use sea_orm::sea_query::Expr;
use sea_orm::ActiveValue::Set;
use sea_orm::{IntoActiveModel, NotSet, QueryOrder, QuerySelect};

use bamboo_entities::prelude::*;
use bamboo_entities::{character, fighter};

use crate::prelude::character_exists;

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
            bamboo_db_error!("fighter", "Failed to load fighters")
        })
}

pub async fn get_fighter(
    id: i32,
    user_id: i32,
    character_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<Fighter> {
    match fighter::Entity::find()
        .filter(fighter::Column::Id.eq(id))
        .filter(fighter::Column::CharacterId.eq(character_id))
        .filter(character::Column::UserId.eq(user_id))
        .inner_join(character::Entity)
        .one(db)
        .await
    {
        Ok(Some(res)) => Ok(res),
        Ok(None) => Err(bamboo_not_found_error!(
            "fighter",
            "The fighter was not found"
        )),
        Err(err) => {
            log::error!("{err}");
            Err(bamboo_db_error!(
                "fighter",
                "Failed to execute database query"
            ))
        }
    }
}

pub async fn fighter_exists(
    id: i32,
    user_id: i32,
    character_id: i32,
    db: &DatabaseConnection,
) -> bool {
    fighter::Entity::find_by_id(id)
        .select_only()
        .column(fighter::Column::Id)
        .filter(fighter::Column::CharacterId.eq(character_id))
        .filter(character::Column::UserId.eq(user_id))
        .inner_join(character::Entity)
        .count(db)
        .await
        .map(|count| count > 0)
        .unwrap_or(false)
}

pub async fn fighter_exists_by_job(
    user_id: i32,
    character_id: i32,
    job: FighterJob,
    db: &DatabaseConnection,
) -> bool {
    fighter::Entity::find()
        .select_only()
        .column(fighter::Column::Id)
        .filter(fighter::Column::Job.eq(job))
        .filter(fighter::Column::CharacterId.eq(character_id))
        .filter(character::Column::UserId.eq(user_id))
        .inner_join(character::Entity)
        .count(db)
        .await
        .map(|count| count > 0)
        .unwrap_or(false)
}

pub async fn create_fighter(
    user_id: i32,
    character_id: i32,
    fighter: Fighter,
    db: &DatabaseConnection,
) -> BambooResult<Fighter> {
    if !character_exists(user_id, character_id, db).await {
        return Err(bamboo_not_found_error!(
            "fighter",
            "The character does not exist"
        ));
    }

    let mut model = fighter.into_active_model();
    model.character_id = Set(character_id);
    model.id = NotSet;

    model.insert(db).await.map_err(|err| {
        log::error!("{err}");
        bamboo_db_error!("fighter", "Failed to create fighter")
    })
}

pub async fn update_fighter(
    id: i32,
    fighter: Fighter,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    fighter::Entity::update_many()
        .filter(fighter::Column::Id.eq(id))
        .col_expr(fighter::Column::Level, Expr::value(fighter.level))
        .col_expr(fighter::Column::GearScore, Expr::value(fighter.gear_score))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            bamboo_db_error!("fighter", "Failed to update fighter")
        })
        .map(|_| ())
}

pub async fn delete_fighter(id: i32, db: &DatabaseConnection) -> BambooErrorResult {
    fighter::Entity::delete_many()
        .filter(fighter::Column::Id.eq(id))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            bamboo_db_error!("fighter", "Failed to delete fighter")
        })
        .map(|_| ())
}

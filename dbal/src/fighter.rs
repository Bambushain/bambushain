use sea_orm::{IntoActiveModel, NotSet, QueryOrder, QuerySelect};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::*;
use sea_orm::sea_query::Expr;

use pandaparty_entities::fighter;
use pandaparty_entities::prelude::*;

pub async fn get_fighters(user_id: i32, db: &DatabaseConnection) -> PandaPartyResult<Vec<Fighter>> {
    fighter::Entity::find()
        .filter(fighter::Column::UserId.eq(user_id))
        .order_by_asc(fighter::Column::Job)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("fighter", "Failed to load fighters")
        })
}

pub async fn get_fighter(id: i32, db: &DatabaseConnection) -> PandaPartyResult<Fighter> {
    match fighter::Entity::find()
        .filter(fighter::Column::Id.eq(id))
        .one(db)
        .await {
        Ok(Some(res)) => Ok(res),
        Ok(None) => Err(pandaparty_not_found_error!("fighter", "The fighter was not found")),
        Err(err) => {
            log::error!("{err}");
            Err(pandaparty_db_error!("fighter", "Failed to execute database query"))
        }
    }
}

pub async fn fighter_exists(id: i32, db: &DatabaseConnection) -> bool {
    match fighter::Entity::find_by_id(id)
        .select_only()
        .column(fighter::Column::Id)
        .count(db)
        .await {
        Ok(count) => count > 0,
        _ => false
    }
}

pub async fn fighter_exists_by_job(user_id: i32, job: String, db: &DatabaseConnection) -> bool {
    match fighter::Entity::find()
        .select_only()
        .column(fighter::Column::Id)
        .filter(fighter::Column::Job.eq(job))
        .filter(fighter::Column::UserId.eq(user_id))
        .count(db)
        .await {
        Ok(count) => count > 0,
        _ => false
    }
}

pub async fn create_fighter(user_id: i32, fighter: Fighter, db: &DatabaseConnection) -> PandaPartyResult<Fighter> {
    let mut model = fighter.into_active_model();
    model.user_id = Set(user_id);
    model.id = NotSet;

    model
        .insert(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("fighter", "Failed to create fighter")
        })
}

pub async fn update_fighter(id: i32, fighter: Fighter, db: &DatabaseConnection) -> PandaPartyErrorResult {
    fighter::Entity::update_many()
        .filter(fighter::Column::Id.eq(id))
        .col_expr(fighter::Column::Job, Expr::value(fighter.job))
        .col_expr(fighter::Column::Level, Expr::value(fighter.level))
        .col_expr(fighter::Column::GearScore, Expr::value(fighter.gear_score))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("fighter", "Failed to update fighter")
        })
        .map(|_| ())
}

pub async fn delete_fighter(id: i32, db: &DatabaseConnection) -> PandaPartyErrorResult {
    fighter::Entity::delete_many()
        .filter(fighter::Column::Id.eq(id))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("fighter", "Failed to delete fighter")
        })
        .map(|_| ())
}

use sea_orm::{IntoActiveModel, QueryOrder};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::*;
use sea_orm::sea_query::Expr;

use sheef_entities::{fighter, sheef_db_error};
use sheef_entities::prelude::*;

pub async fn get_fighters(username: String) -> SheefResult<Vec<Fighter>> {
    let db = open_db_connection!();
    let user = get_user_by_username!(username);

    fighter::Entity::find()
        .filter(fighter::Column::UserId.eq(user.id))
        .order_by_asc(fighter::Column::Job)
        .all(&db)
        .await
        .map_err(|_| sheef_db_error!("fighter", "Failed to load fighters"))
}

pub async fn get_fighter(username: String, job: String) -> SheefResult<Fighter> {
    let db = open_db_connection!();
    let user = get_user_by_username!(username);

    match fighter::Entity::find()
        .filter(fighter::Column::Job.eq(job))
        .filter(fighter::Column::UserId.eq(user.id))
        .one(&db)
        .await {
        Ok(Some(res)) => Ok(res),
        Ok(None) => Err(sheef_not_found_error!("fighter", "The fighter was not found")),
        Err(_) => Err(sheef_db_error!("fighter", "Failed to execute database query"))
    }
}

pub async fn fighter_exists(username: String, job: String) -> bool {
    get_fighter(username, job).await.is_ok()
}

pub async fn create_fighter(username: String, fighter: Fighter) -> SheefResult<Fighter> {
    let db = open_db_connection!();
    let user = get_user_by_username!(username);

    let mut model = fighter.into_active_model();
    model.user_id = Set(user.id);

    model.insert(&db).await.map_err(|_| sheef_db_error!("fighter", "Failed to create fighter"))
}

pub async fn update_fighter(username: String, job: String, fighter: Fighter) -> SheefErrorResult {
    let db = open_db_connection!();
    let user = get_user_by_username!(username);

    fighter::Entity::update_many()
        .filter(fighter::Column::Job.eq(job))
        .filter(fighter::Column::UserId.eq(user.id))
        .col_expr(fighter::Column::Job, Expr::value(fighter.job))
        .col_expr(fighter::Column::Level, Expr::value(fighter.level))
        .col_expr(fighter::Column::GearScore, Expr::value(fighter.gear_score))
        .exec(&db)
        .await
        .map_err(|_| sheef_db_error!("fighter", "Failed to update fighter"))
        .map(|_| ())
}

pub async fn delete_fighter(username: String, job: String) -> SheefErrorResult {
    let db = open_db_connection!();
    let user = get_user_by_username!(username);

    fighter::Entity::delete_many()
        .filter(fighter::Column::Job.eq(job))
        .filter(fighter::Column::UserId.eq(user.id))
        .exec(&db)
        .await
        .map_err(|_| sheef_db_error!("fighter", "Failed to delete fighter"))
        .map(|_| ())
}

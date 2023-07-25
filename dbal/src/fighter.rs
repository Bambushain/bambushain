use sea_orm::{IntoActiveModel, JoinType, NotSet, QueryOrder, QuerySelect};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::*;
use sea_orm::sea_query::Expr;

use sheef_entities::{fighter, sheef_db_error, user};
use sheef_entities::prelude::*;

pub async fn get_fighters(username: String) -> SheefResult<Vec<Fighter>> {
    let db = open_db_connection!();

    let result = fighter::Entity::find()
        .filter(user::Column::Username.eq(username))
        .join(JoinType::InnerJoin, fighter::Relation::User.def())
        .order_by_asc(fighter::Column::Job)
        .all(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            sheef_db_error!("fighter", "Failed to load fighters")
        });

    let _ = db.close().await;

    result
}

pub async fn get_fighter(username: String, job: String) -> SheefResult<Fighter> {
    let db = open_db_connection!();

    let result = match fighter::Entity::find()
        .filter(fighter::Column::Job.eq(job))
        .filter(user::Column::Username.eq(username))
        .join(JoinType::InnerJoin, fighter::Relation::User.def())
        .one(&db)
        .await {
        Ok(Some(res)) => Ok(res),
        Ok(None) => Err(sheef_not_found_error!("fighter", "The fighter was not found")),
        Err(err) => {
            log::error!("{err}");
            Err(sheef_db_error!("fighter", "Failed to execute database query"))
        }
    };

    let _ = db.close().await;

    result
}

pub async fn fighter_exists(username: String, job: String) -> bool {
    get_fighter(username, job).await.is_ok()
}

pub async fn create_fighter(username: String, fighter: Fighter) -> SheefResult<Fighter> {
    let db = open_db_connection!();
    let user = match crate::user::get_user(username.clone()).await {
        Ok(user) => user,
        Err(err) => {
            log::error!("Failed to load user {}: {err}", username);
            return Err(err);
        }
    };

    let mut model = fighter.into_active_model();
    model.user_id = Set(user.id);
    model.id = NotSet;

    let result = model
        .insert(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            sheef_db_error!("fighter", "Failed to create fighter")
        });

    let _ = db.close().await;

    result
}

pub async fn update_fighter(username: String, job: String, fighter: Fighter) -> SheefErrorResult {
    let db = open_db_connection!();
    let user = match crate::user::get_user(username.clone()).await {
        Ok(user) => user,
        Err(err) => {
            log::error!("Failed to load user {}: {err}", username);
            return Err(err);
        }
    };

    let result = fighter::Entity::update_many()
        .filter(fighter::Column::Job.eq(job))
        .filter(fighter::Column::UserId.eq(user.id))
        .col_expr(fighter::Column::Job, Expr::value(fighter.job))
        .col_expr(fighter::Column::Level, Expr::value(fighter.level))
        .col_expr(fighter::Column::GearScore, Expr::value(fighter.gear_score))
        .exec(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            sheef_db_error!("fighter", "Failed to update fighter")
        })
        .map(|_| ());

    let _ = db.close().await;

    result
}

pub async fn delete_fighter(username: String, job: String) -> SheefErrorResult {
    let db = open_db_connection!();
    let user = match crate::user::get_user(username.clone()).await {
        Ok(user) => user,
        Err(err) => {
            log::error!("Failed to load user {}: {err}", username);
            return Err(err);
        }
    };

    let result = fighter::Entity::delete_many()
        .filter(fighter::Column::Job.eq(job))
        .filter(fighter::Column::UserId.eq(user.id))
        .exec(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            sheef_db_error!("fighter", "Failed to delete fighter")
        })
        .map(|_| ());

    let _ = db.close().await;

    result
}

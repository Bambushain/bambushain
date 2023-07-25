use sea_orm::{IntoActiveModel, NotSet, QueryOrder};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::*;
use sea_orm::sea_query::Expr;

use sheef_entities::{crafter, sheef_db_error};
use sheef_entities::prelude::*;

pub async fn get_crafters(username: String) -> SheefResult<Vec<Crafter>> {
    let db = open_db_connection!();
    let user = get_user_by_username!(username);

    crafter::Entity::find()
        .filter(crafter::Column::UserId.eq(user.id))
        .order_by_asc(crafter::Column::Job)
        .all(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            sheef_db_error!("crafter", "Failed to load crafters")
        })
}

pub async fn get_crafter(username: String, job: String) -> SheefResult<Crafter> {
    let db = open_db_connection!();
    let user = get_user_by_username!(username);

    match crafter::Entity::find()
        .filter(crafter::Column::Job.eq(job))
        .filter(crafter::Column::UserId.eq(user.id))
        .one(&db)
        .await {
        Ok(Some(res)) => Ok(res),
        Ok(None) => Err(sheef_not_found_error!("crafter", "The crafter was not found")),
        Err(err) => {
            log::error!("{err}");
            Err(sheef_db_error!("crafter", "Failed to execute database query"))
        }
    }
}

pub async fn crafter_exists(username: String, job: String) -> bool {
    get_crafter(username, job).await.is_ok()
}

pub async fn create_crafter(username: String, crafter: Crafter) -> SheefResult<Crafter> {
    let db = open_db_connection!();
    let user = get_user_by_username!(username);

    let mut model = crafter.into_active_model();
    model.user_id = Set(user.id);
    model.id = NotSet;

    model
        .insert(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            sheef_db_error!("crafter", "Failed to create crafter")
        })
}

pub async fn update_crafter(username: String, job: String, crafter: Crafter) -> SheefErrorResult {
    let db = open_db_connection!();
    let user = get_user_by_username!(username);

    crafter::Entity::update_many()
        .filter(crafter::Column::Job.eq(job))
        .filter(crafter::Column::UserId.eq(user.id))
        .col_expr(crafter::Column::Job, Expr::value(crafter.job))
        .col_expr(crafter::Column::Level, Expr::value(crafter.level))
        .exec(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            sheef_db_error!("crafter", "Failed to update crafter")
        })
        .map(|_| ())
}

pub async fn delete_crafter(username: String, job: String) -> SheefErrorResult {
    let db = open_db_connection!();
    let user = get_user_by_username!(username);

    crafter::Entity::delete_many()
        .filter(crafter::Column::Job.eq(job))
        .filter(crafter::Column::UserId.eq(user.id))
        .exec(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            sheef_db_error!("crafter", "Failed to delete crafter")
        })
        .map(|_| ())
}

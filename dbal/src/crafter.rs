use sea_orm::{IntoActiveModel, JoinType, NotSet, QueryOrder, QuerySelect};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::*;
use sea_orm::sea_query::Expr;

use sheef_entities::{crafter, sheef_db_error, user};
use sheef_entities::prelude::*;

pub async fn get_crafters(username: String) -> SheefResult<Vec<Crafter>> {
    let db = open_db_connection!();

    let result = crafter::Entity::find()
        .filter(user::Column::Username.eq(username))
        .join(JoinType::InnerJoin, crafter::Relation::User.def())
        .order_by_asc(crafter::Column::Job)
        .all(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            sheef_db_error!("crafter", "Failed to load crafters")
        });

    let _ = db.close().await;

    result
}

pub async fn get_crafter(username: String, job: String) -> SheefResult<Crafter> {
    let db = open_db_connection!();

    let result = match crafter::Entity::find()
        .filter(crafter::Column::Job.eq(job))
        .filter(user::Column::Username.eq(username))
        .join(JoinType::InnerJoin, crafter::Relation::User.def())
        .one(&db)
        .await {
        Ok(Some(res)) => Ok(res),
        Ok(None) => Err(sheef_not_found_error!("crafter", "The crafter was not found")),
        Err(err) => {
            log::error!("{err}");
            Err(sheef_db_error!("crafter", "Failed to execute database query"))
        }
    };

    let _ = db.close().await;

    result
}

pub async fn crafter_exists(username: String, job: String) -> bool {
    get_crafter(username, job).await.is_ok()
}

pub async fn create_crafter(username: String, crafter: Crafter) -> SheefResult<Crafter> {
    let db = open_db_connection!();
    let user = match crate::user::get_user(username.clone()).await {
        Ok(user) => user,
        Err(err) => {
            log::error!("Failed to load user {}: {err}", username);
            return Err(err);
        }
    };

    let mut model = crafter.into_active_model();
    model.user_id = Set(user.id);
    model.id = NotSet;

    let result = model
        .insert(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            sheef_db_error!("crafter", "Failed to create crafter")
        });

    let _ = db.close().await;

    result
}

pub async fn update_crafter(username: String, job: String, crafter: Crafter) -> SheefErrorResult {
    let db = open_db_connection!();
    let user = match crate::user::get_user(username.clone()).await {
        Ok(user) => user,
        Err(err) => {
            log::error!("Failed to load user {}: {err}", username);
            return Err(err);
        }
    };

    let result = crafter::Entity::update_many()
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
        .map(|_| ());

    let _ = db.close().await;

    result
}

pub async fn delete_crafter(username: String, job: String) -> SheefErrorResult {
    let db = open_db_connection!();
    let user = match crate::user::get_user(username.clone()).await {
        Ok(user) => user,
        Err(err) => {
            log::error!("Failed to load user {}: {err}", username);
            return Err(err);
        }
    };

    let result = crafter::Entity::delete_many()
        .filter(crafter::Column::Job.eq(job))
        .filter(crafter::Column::UserId.eq(user.id))
        .exec(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            sheef_db_error!("crafter", "Failed to delete crafter")
        })
        .map(|_| ());

    let _ = db.close().await;

    result
}

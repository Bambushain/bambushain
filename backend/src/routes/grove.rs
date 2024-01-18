use actix_web::{delete, get, put};

use bamboo_dbal::prelude::dbal;
use bamboo_entities::prelude::Grove;
use bamboo_error::{BambooApiResponseResult, BambooApiResult};
use bamboo_services::prelude::DbConnection;

use crate::middleware::authenticate_user::authenticate;
use crate::middleware::check_mod::is_mod;
use crate::middleware::identify_grove::{grove, CurrentGrove};
use crate::response::macros::{no_content, ok};

#[get("/api/grove", wrap = "authenticate!()", wrap = "grove!()")]
pub async fn get_grove(current_grove: CurrentGrove) -> BambooApiResult<Grove> {
    Ok(ok!(current_grove.grove.clone()))
}

#[delete(
    "/api/grove/enabled",
    wrap = "authenticate!()",
    wrap = "grove!()",
    wrap = "is_mod!()"
)]
pub async fn disable_grove(
    current_grove: CurrentGrove,
    db: DbConnection,
) -> BambooApiResponseResult {
    dbal::disable_grove(current_grove.grove.id, &db)
        .await
        .map(|_| no_content!())
}

#[put(
    "/api/grove/enabled",
    wrap = "authenticate!()",
    wrap = "grove!()",
    wrap = "is_mod!()"
)]
pub async fn enable_grove(
    current_grove: CurrentGrove,
    db: DbConnection,
) -> BambooApiResponseResult {
    dbal::enable_grove(current_grove.grove.id, &db)
        .await
        .map(|_| no_content!())
}

#[delete(
    "/api/grove",
    wrap = "authenticate!()",
    wrap = "grove!()",
    wrap = "is_mod!()"
)]
pub async fn delete_grove(
    current_grove: CurrentGrove,
    db: DbConnection,
) -> BambooApiResponseResult {
    dbal::delete_grove(current_grove.grove.id, &db)
        .await
        .map(|_| no_content!())
}

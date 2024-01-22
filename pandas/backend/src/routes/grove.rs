use actix_web::{delete, get, put};

use bamboo_common::core::entities::Grove;
use bamboo_common::core::error::BambooApiResult;

use crate::middleware::authenticate_user::authenticate;
use crate::middleware::identify_grove::{CurrentGrove, grove};
use crate::response::macros::ok;

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

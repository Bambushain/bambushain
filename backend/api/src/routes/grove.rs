use actix_web::{delete, get, post, put, web};

use bamboo_common::backend::dbal;
use bamboo_common::backend::response::*;
use bamboo_common::backend::services::DbConnection;
use bamboo_common::core::entities::grove::{CreateGrove, JoinGrove};
use bamboo_common::core::entities::user::JoinStatus;
use bamboo_common::core::entities::*;
use bamboo_common::core::error::*;

use crate::middleware::check_grove_mod::grove_mod;
use crate::path;
use bamboo_common::backend::actix::middleware::{authenticate, Authentication};

#[get("/api/grove", wrap = "authenticate!()")]
pub async fn get_groves(
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    dbal::get_groves(authentication.user.id, &db)
        .await
        .map(|data| list!(data))
}

#[get("/api/grove/{grove_id}", wrap = "authenticate!()")]
pub async fn get_grove(
    path: Option<path::GrovePath>,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResult<Grove> {
    let path = check_invalid_path!(path, "grove")?;

    dbal::get_grove(path.grove_id, authentication.user.id, &db)
        .await
        .map(|data| ok!(data))
}

#[post("/api/grove", wrap = "authenticate!()")]
pub async fn create_grove(
    body: Option<web::Json<CreateGrove>>,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResult<Grove> {
    let body = check_missing_fields!(body, "grove")?;

    dbal::create_grove(
        body.name.clone(),
        body.invite_on,
        authentication.user.id,
        &db,
    )
    .await
    .map(|data| created!(data))
}

#[put(
    "/api/grove/{grove_id}",
    wrap = "authenticate!()",
    wrap = "grove_mod!()"
)]
pub async fn update_grove(
    body: Option<web::Json<Grove>>,
    path: Option<path::GrovePath>,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "grove")?;
    let body = check_missing_fields!(body, "grove")?;

    dbal::update_grove(
        path.grove_id,
        authentication.user.id,
        body.name.clone(),
        &db,
    )
    .await
    .map(|_| no_content!())
}

#[put(
    "/api/grove/{grove_id}/mods",
    wrap = "authenticate!()",
    wrap = "grove_mod!()"
)]
pub async fn update_grove_mods(
    body: Option<web::Json<Vec<i32>>>,
    path: Option<path::GrovePath>,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "grove")?;
    let body = check_missing_fields!(body, "grove")?;

    dbal::update_grove_mods(
        path.grove_id,
        authentication.user.id,
        body.into_inner(),
        &db,
    )
    .await
    .map(|_| no_content!())
}

#[delete(
    "/api/grove/{grove_id}",
    wrap = "authenticate!()",
    wrap = "grove_mod!()"
)]
pub async fn delete_grove(
    path: Option<path::GrovePath>,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "grove")?;

    dbal::delete_grove(path.grove_id, authentication.user.id, &db)
        .await
        .map(|_| no_content!())
}

#[put(
    "/api/grove/{grove_id}/user/{user_id}/ban",
    wrap = "authenticate!()",
    wrap = "grove_mod!()"
)]
pub async fn ban_user(
    path: Option<path::GroveUserPath>,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "grove")?;

    dbal::ban_user_from_grove(path.grove_id, path.user_id, &db)
        .await
        .map(|_| no_content!())
}

#[delete(
    "/api/grove/{grove_id}/user/{user_id}/ban",
    wrap = "authenticate!()",
    wrap = "grove_mod!()"
)]
pub async fn unban_user(
    path: Option<path::GroveUserPath>,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "grove")?;

    dbal::unban_user_from_grove(path.grove_id, path.user_id, &db)
        .await
        .map(|_| no_content!())
}

#[put(
    "/api/grove/{grove_id}/invite",
    wrap = "authenticate!()",
    wrap = "grove_mod!()"
)]
pub async fn enable_invite(
    path: Option<path::GrovePath>,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "grove")?;

    dbal::enable_grove_invite(path.grove_id, &db)
        .await
        .map(|_| no_content!())
}

#[delete(
    "/api/grove/{grove_id}/invite",
    wrap = "authenticate!()",
    wrap = "grove_mod!()"
)]
pub async fn disable_invite(
    path: Option<path::GrovePath>,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "grove")?;

    dbal::disable_grove_invite(path.grove_id, &db)
        .await
        .map(|_| no_content!())
}

#[post("/api/grove/{grove_id}/join", wrap = "authenticate!()")]
pub async fn join_grove(
    path: Option<path::GrovePath>,
    body: Option<web::Json<JoinGrove>>,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    let body = check_missing_fields!(body, "grove")?;
    let path = check_invalid_path!(path, "grove")?;

    dbal::join_grove(
        path.grove_id,
        authentication.user.id,
        body.invite_secret.clone(),
        &db,
    )
    .await
    .map(|_| no_content!())
}

#[get("/api/grove/{grove_id}/join", wrap = "authenticate!()")]
pub async fn check_join_status(
    path: Option<path::GrovePath>,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResult<JoinStatus> {
    let path = check_invalid_path!(path, "grove")?;

    dbal::check_grove_join_status(path.grove_id, authentication.user.id, &db)
        .await
        .map(|data| ok!(data))
}

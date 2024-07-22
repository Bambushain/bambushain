use actix_web::{delete, get, post, put, web};

use bamboo_common::backend::dbal;
use bamboo_common::backend::response::*;
use bamboo_common::backend::services::DbConnection;
use bamboo_common::core::entities::*;
use bamboo_common::core::error::*;

use crate::middleware::authenticate_user::{authenticate, Authentication};
use crate::path;

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
    body: Option<web::Json<Grove>>,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResult<Grove> {
    let body = check_missing_fields!(body, "grove")?;

    dbal::create_grove(body.name.clone(), authentication.user.id, &db)
        .await
        .map(|data| created!(data))
}

#[put("/api/grove/{grove_id}", wrap = "authenticate!()")]
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

#[delete("/api/grove/{grove_id}", wrap = "authenticate!()")]
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

use actix_web::{delete, get, post, put, web};

use bamboo_dbal::prelude::dbal;
use bamboo_entities::prelude::*;
use bamboo_error::*;
use bamboo_services::prelude::DbConnection;

use crate::middleware::authenticate_user::{authenticate, Authentication};
use crate::path;
use crate::response::macros::*;

#[get("/api/final-fantasy/free-company", wrap = "authenticate!()")]
pub async fn get_free_companies(
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    dbal::get_free_companies(authentication.user.id, &db)
        .await
        .map(|data| list!(data))
}

#[get(
    "/api/final-fantasy/free-company/{free_company_id}",
    wrap = "authenticate!()"
)]
pub async fn get_free_company(
    path: Option<path::FreeCompanyPath>,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResult<FreeCompany> {
    let path = check_invalid_path!(path, "free_company")?;

    dbal::get_free_company(Some(path.free_company_id), authentication.user.id, &db)
        .await
        .map(|data| ok!(data.unwrap()))
}

#[post("/api/final-fantasy/free-company", wrap = "authenticate!()")]
pub async fn create_free_company(
    body: Option<web::Json<FreeCompany>>,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResult<FreeCompany> {
    let body = check_missing_fields!(body, "free_company")?;

    dbal::create_free_company(authentication.user.id, body.name.clone(), &db)
        .await
        .map(|data| created!(data))
}

#[put(
    "/api/final-fantasy/free-company/{free_company_id}",
    wrap = "authenticate!()"
)]
pub async fn update_free_company(
    body: Option<web::Json<FreeCompany>>,
    path: Option<path::FreeCompanyPath>,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "free_company")?;
    let body = check_missing_fields!(body, "free_company")?;

    dbal::update_free_company(
        path.free_company_id,
        authentication.user.id,
        body.name.clone(),
        &db,
    )
    .await
    .map(|_| no_content!())
}

#[delete(
    "/api/final-fantasy/free-company/{free_company_id}",
    wrap = "authenticate!()"
)]
pub async fn delete_free_company(
    path: Option<path::FreeCompanyPath>,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "free_company")?;

    dbal::delete_free_company(path.free_company_id, authentication.user.id, &db)
        .await
        .map(|_| no_content!())
}

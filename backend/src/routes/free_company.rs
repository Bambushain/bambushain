use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::Deserialize;

use bamboo_entities::prelude::*;
use bamboo_error::*;
use bamboo_services::prelude::DbConnection;

use crate::middleware::authenticate_user::{authenticate, Authentication};

#[derive(Deserialize)]
pub struct FreeCompanyPathInfo {
    pub id: i32,
}

#[get("/api/final-fantasy/free-company", wrap = "authenticate!()")]
pub async fn get_free_companies(authentication: Authentication, db: DbConnection) -> HttpResponse {
    ok_or_error!(bamboo_dbal::free_company::get_free_companies(authentication.user.id, &db).await)
}

#[get("/api/final-fantasy/free-company/{id}", wrap = "authenticate!()")]
pub async fn get_free_company(
    path: Option<web::Path<FreeCompanyPathInfo>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    let path = check_invalid_path!(path, "free_company");

    match bamboo_dbal::free_company::get_free_company(Some(path.id), authentication.user.id, &db)
        .await
    {
        Ok(free_company) => ok_json!(free_company),
        Err(_) => not_found!(bamboo_not_found_error!(
            "free_company",
            "The free company was not found"
        )),
    }
}

#[post("/api/final-fantasy/free-company", wrap = "authenticate!()")]
pub async fn create_free_company(
    body: Option<web::Json<FreeCompany>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    let body = check_missing_fields!(body, "free_company");

    if bamboo_dbal::free_company::free_company_exists_by_name(
        body.name.clone(),
        authentication.user.id,
        &db,
    )
    .await
    {
        return conflict!(bamboo_exists_already_error!(
            "free_company",
            "The free company already exists"
        ));
    }

    created_or_error!(
        bamboo_dbal::free_company::create_free_company(
            authentication.user.id,
            body.name.clone(),
            &db
        )
        .await
    )
}

#[put("/api/final-fantasy/free-company/{id}", wrap = "authenticate!()")]
pub async fn update_free_company(
    body: Option<web::Json<FreeCompany>>,
    path: Option<web::Path<FreeCompanyPathInfo>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    let path = check_invalid_path!(path, "free_company");
    let body = check_missing_fields!(body, "free_company");

    match bamboo_dbal::free_company::get_free_company(Some(path.id), authentication.user.id, &db)
        .await
    {
        Ok(_) => no_content_or_error!(
            bamboo_dbal::free_company::update_free_company(
                path.id,
                authentication.user.id,
                body.name.clone(),
                &db
            )
            .await
        ),
        Err(_) => not_found!(bamboo_not_found_error!(
            "free_company",
            "The free company was not found"
        )),
    }
}

#[delete("/api/final-fantasy/free-company/{id}", wrap = "authenticate!()")]
pub async fn delete_free_company(
    path: Option<web::Path<FreeCompanyPathInfo>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    let path = check_invalid_path!(path, "free_company");

    if !bamboo_dbal::free_company::free_company_exists(authentication.user.id, path.id, &db).await {
        return not_found!(bamboo_not_found_error!(
            "free_company",
            "The free company was not found"
        ));
    }

    no_content_or_error!(
        bamboo_dbal::free_company::delete_free_company(path.id, authentication.user.id, &db).await
    )
}

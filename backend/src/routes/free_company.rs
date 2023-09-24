use actix_web::{web, HttpResponse};
use serde::Deserialize;

use pandaparty_entities::prelude::*;

use crate::middleware::authenticate_user::Authentication;
use crate::DbConnection;

#[derive(Deserialize)]
pub struct FreeCompanyPathInfo {
    pub id: i32,
}

pub async fn get_free_companies(authentication: Authentication, db: DbConnection) -> HttpResponse {
    ok_or_error!(
        pandaparty_dbal::free_company::get_free_companies(authentication.user.id, &db).await
    )
}

pub async fn get_free_company(
    path: web::Path<FreeCompanyPathInfo>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    match pandaparty_dbal::free_company::get_free_company(
        Some(path.id),
        authentication.user.id,
        &db,
    )
    .await
    {
        Ok(free_company) => ok_json!(free_company),
        Err(_) => not_found!(pandaparty_not_found_error!(
            "free_company",
            "The free company was not found"
        )),
    }
}

pub async fn create_free_company(
    body: web::Json<FreeCompany>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    if pandaparty_dbal::free_company::free_company_exists_by_name(
        body.name.clone(),
        authentication.user.id,
        &db,
    )
    .await
    {
        return conflict!(pandaparty_exists_already_error!(
            "free_company",
            "The free company already exists"
        ));
    }

    created_or_error!(
        pandaparty_dbal::free_company::create_free_company(
            authentication.user.id,
            body.name.clone(),
            &db
        )
        .await
    )
}

pub async fn update_free_company(
    body: web::Json<FreeCompany>,
    path: web::Path<FreeCompanyPathInfo>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    match pandaparty_dbal::free_company::get_free_company(
        Some(path.id),
        authentication.user.id,
        &db,
    )
    .await
    {
        Ok(_) => no_content_or_error!(
            pandaparty_dbal::free_company::update_free_company(
                path.id,
                authentication.user.id,
                body.name.clone(),
                &db
            )
            .await
        ),
        Err(_) => not_found!(pandaparty_not_found_error!(
            "free_company",
            "The free company was not found"
        )),
    }
}

pub async fn delete_free_company(
    path: web::Path<FreeCompanyPathInfo>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    if !pandaparty_dbal::free_company::free_company_exists(authentication.user.id, path.id, &db)
        .await
    {
        return not_found!(pandaparty_not_found_error!(
            "free_company",
            "The free company was not found"
        ));
    }

    no_content_or_error!(
        pandaparty_dbal::free_company::delete_free_company(path.id, authentication.user.id, &db)
            .await
    )
}

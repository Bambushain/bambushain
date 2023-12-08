use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::Deserialize;

use bamboo_entities::prelude::*;
use bamboo_error::*;
use bamboo_services::prelude::DbConnection;

use crate::middleware::authenticate_user::{authenticate, Authentication};

#[derive(Deserialize)]
pub struct CraftersPathInfo {
    pub character_id: i32,
}

#[derive(Deserialize)]
pub struct CrafterPathInfo {
    pub id: i32,
    pub character_id: i32,
}

#[get(
    "/api/final-fantasy/character/{character_id}/crafter",
    wrap = "authenticate!()"
)]
pub async fn get_crafters(
    path: Option<web::Path<CraftersPathInfo>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    let path = check_invalid_path!(path, "crafter");

    ok_or_error!(
        bamboo_dbal::crafter::get_crafters(authentication.user.id, path.character_id, &db).await
    )
}

#[get(
    "/api/final-fantasy/character/{character_id}/crafter/{id}",
    wrap = "authenticate!()"
)]
pub async fn get_crafter(
    path: Option<web::Path<CrafterPathInfo>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    let path = check_invalid_path!(path, "crafter");

    match bamboo_dbal::crafter::get_crafter(path.id, authentication.user.id, path.character_id, &db)
        .await
    {
        Ok(crafter) => ok_json!(crafter),
        Err(_) => not_found!(bamboo_not_found_error!(
            "crafter",
            "The crafter was not found"
        )),
    }
}

#[post(
    "/api/final-fantasy/character/{character_id}/crafter",
    wrap = "authenticate!()"
)]
pub async fn create_crafter(
    path: Option<web::Path<CraftersPathInfo>>,
    body: Option<web::Json<Crafter>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    let path = check_invalid_path!(path, "crafter");
    let body = check_missing_fields!(body, "crafter");

    if bamboo_dbal::crafter::crafter_exists_by_job(
        authentication.user.id,
        path.character_id,
        body.job,
        &db,
    )
    .await
    {
        return conflict!(bamboo_exists_already_error!(
            "crafter",
            "The crafter already exists"
        ));
    }

    created_or_error!(
        bamboo_dbal::crafter::create_crafter(
            authentication.user.id,
            path.character_id,
            body.into_inner(),
            &db
        )
        .await
    )
}

#[put(
    "/api/final-fantasy/character/{character_id}/crafter/{id}",
    wrap = "authenticate!()"
)]
pub async fn update_crafter(
    body: Option<web::Json<Crafter>>,
    path: Option<web::Path<CrafterPathInfo>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    let path = check_invalid_path!(path, "crafter");
    let body = check_missing_fields!(body, "crafter");

    match bamboo_dbal::crafter::get_crafter(path.id, authentication.user.id, path.character_id, &db)
        .await
    {
        Ok(_) => no_content_or_error!(
            bamboo_dbal::crafter::update_crafter(path.id, body.into_inner(), &db).await
        ),
        Err(_) => not_found!(bamboo_not_found_error!(
            "crafter",
            "The crafter was not found"
        )),
    }
}

#[delete(
    "/api/final-fantasy/character/{character_id}/crafter/{id}",
    wrap = "authenticate!()"
)]
pub async fn delete_crafter(
    path: Option<web::Path<CrafterPathInfo>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    let path = check_invalid_path!(path, "crafter");

    if !bamboo_dbal::crafter::crafter_exists(
        path.id,
        authentication.user.id,
        path.character_id,
        &db,
    )
    .await
    {
        return not_found!(bamboo_not_found_error!(
            "crafter",
            "The crafter was not found"
        ));
    }

    no_content_or_error!(bamboo_dbal::crafter::delete_crafter(path.id, &db).await)
}

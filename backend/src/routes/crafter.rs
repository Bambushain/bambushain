use actix_web::{web, HttpResponse};
use serde::Deserialize;

use bamboo_entities::prelude::*;

use crate::middleware::authenticate_user::Authentication;
use crate::DbConnection;

#[derive(Deserialize)]
pub struct CraftersPathInfo {
    pub character_id: i32,
}

#[derive(Deserialize)]
pub struct CrafterPathInfo {
    pub id: i32,
    pub character_id: i32,
}

pub async fn get_crafters(
    path: web::Path<CraftersPathInfo>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    ok_or_error!(
        bamboo_dbal::crafter::get_crafters(authentication.user.id, path.character_id, &db).await
    )
}

pub async fn get_crafter(
    path: web::Path<CrafterPathInfo>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
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

pub async fn create_crafter(
    path: web::Path<CraftersPathInfo>,
    body: web::Json<Crafter>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
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

pub async fn update_crafter(
    body: web::Json<Crafter>,
    path: web::Path<CrafterPathInfo>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
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

pub async fn delete_crafter(
    path: web::Path<CrafterPathInfo>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
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

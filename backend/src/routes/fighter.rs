use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::Deserialize;

use bamboo_entities::prelude::*;
use bamboo_error::*;
use bamboo_services::prelude::DbConnection;

use crate::middleware::authenticate_user::{authenticate, Authentication};

#[derive(Deserialize)]
pub struct FightersPathInfo {
    pub character_id: i32,
}

#[derive(Deserialize)]
pub struct FighterPathInfo {
    pub id: i32,
    pub character_id: i32,
}

#[get(
    "/api/final-fantasy/character/{character_id}/fighter",
    wrap = "authenticate!()"
)]
pub async fn get_fighters(
    path: Option<web::Path<FightersPathInfo>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    let path = check_invalid_path!(path, "fighter");

    ok_or_error!(
        bamboo_dbal::fighter::get_fighters(authentication.user.id, path.character_id, &db).await
    )
}

#[get(
    "/api/final-fantasy/character/{character_id}/fighter/{id}",
    wrap = "authenticate!()"
)]
pub async fn get_fighter(
    path: Option<web::Path<FighterPathInfo>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    let path = check_invalid_path!(path, "fighter");

    match bamboo_dbal::fighter::get_fighter(path.id, authentication.user.id, path.character_id, &db)
        .await
    {
        Ok(fighter) => ok_json!(fighter),
        Err(_) => not_found!(bamboo_not_found_error!(
            "fighter",
            "The fighter was not found"
        )),
    }
}

#[post(
    "/api/final-fantasy/character/{character_id}/fighter",
    wrap = "authenticate!()"
)]
pub async fn create_fighter(
    path: Option<web::Path<FightersPathInfo>>,
    body: Option<web::Json<Fighter>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    let path = check_invalid_path!(path, "fighter");
    let body = check_missing_fields!(body, "fighter");

    if bamboo_dbal::fighter::fighter_exists_by_job(
        authentication.user.id,
        path.character_id,
        body.job,
        &db,
    )
    .await
    {
        return conflict!(bamboo_exists_already_error!(
            "fighter",
            "The fighter already exists"
        ));
    }

    created_or_error!(
        bamboo_dbal::fighter::create_fighter(
            authentication.user.id,
            path.character_id,
            body.into_inner(),
            &db
        )
        .await
    )
}

#[put(
    "/api/final-fantasy/character/{character_id}/fighter/{id}",
    wrap = "authenticate!()"
)]
pub async fn update_fighter(
    body: Option<web::Json<Fighter>>,
    path: Option<web::Path<FighterPathInfo>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    let path = check_invalid_path!(path, "fighter");
    let body = check_missing_fields!(body, "fighter");

    match bamboo_dbal::fighter::get_fighter(path.id, authentication.user.id, path.character_id, &db)
        .await
    {
        Ok(_) => no_content_or_error!(
            bamboo_dbal::fighter::update_fighter(path.id, body.into_inner(), &db).await
        ),
        Err(_) => not_found!(bamboo_not_found_error!(
            "fighter",
            "The fighter was not found"
        )),
    }
}

#[delete(
    "/api/final-fantasy/character/{character_id}/fighter/{id}",
    wrap = "authenticate!()"
)]
pub async fn delete_fighter(
    path: Option<web::Path<FighterPathInfo>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    let path = check_invalid_path!(path, "fighter");

    if !bamboo_dbal::fighter::fighter_exists(
        path.id,
        authentication.user.id,
        path.character_id,
        &db,
    )
    .await
    {
        return not_found!(bamboo_not_found_error!(
            "fighter",
            "The fighter was not found"
        ));
    }

    no_content_or_error!(bamboo_dbal::fighter::delete_fighter(path.id, &db).await)
}

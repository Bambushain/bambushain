use actix_web::{web, HttpResponse};
use serde::Deserialize;

use bamboo_entities::prelude::*;

use crate::middleware::authenticate_user::Authentication;
use crate::DbConnection;

#[derive(Deserialize)]
pub struct FightersPathInfo {
    pub character_id: i32,
}

#[derive(Deserialize)]
pub struct FighterPathInfo {
    pub id: i32,
    pub character_id: i32,
}

pub async fn get_fighters(
    path: web::Path<FightersPathInfo>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    ok_or_error!(
        bamboo_dbal::fighter::get_fighters(authentication.user.id, path.character_id, &db).await
    )
}

pub async fn get_fighter(
    path: web::Path<FighterPathInfo>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
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

pub async fn create_fighter(
    path: web::Path<FightersPathInfo>,
    body: web::Json<Fighter>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
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

pub async fn update_fighter(
    body: web::Json<Fighter>,
    path: web::Path<FighterPathInfo>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
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

pub async fn delete_fighter(
    path: web::Path<FighterPathInfo>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
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

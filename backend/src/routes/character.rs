use actix_web::{web, HttpResponse};
use serde::Deserialize;

use bamboo_entities::prelude::*;
use bamboo_error::*;
use bamboo_services::prelude::DbConnection;

use crate::middleware::authenticate_user::Authentication;

#[derive(Deserialize)]
pub struct CharacterPathInfo {
    pub id: i32,
}

pub async fn get_characters(authentication: Authentication, db: DbConnection) -> HttpResponse {
    ok_or_error!(bamboo_dbal::character::get_characters(authentication.user.id, &db).await)
}

pub async fn get_character(
    path: Option<web::Path<CharacterPathInfo>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    let path = check_invalid_path!(path, "character");

    match bamboo_dbal::character::get_character(path.id, authentication.user.id, &db).await {
        Ok(character) => ok_json!(character),
        Err(_) => not_found!(bamboo_not_found_error!(
            "character",
            "The character was not found"
        )),
    }
}

pub async fn create_character(
    body: Option<web::Json<Character>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    let body = check_missing_fields!(body, "character");

    if bamboo_dbal::character::character_exists_by_name(
        body.name.clone(),
        authentication.user.id,
        &db,
    )
    .await
    {
        return conflict!(bamboo_exists_already_error!(
            "character",
            "The character already exists"
        ));
    }

    created_or_error!(
        bamboo_dbal::character::create_character(authentication.user.id, body.into_inner(), &db)
            .await
    )
}

pub async fn update_character(
    body: Option<web::Json<Character>>,
    path: Option<web::Path<CharacterPathInfo>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    let path = check_invalid_path!(path, "character");
    let body = check_missing_fields!(body, "character");

    match bamboo_dbal::character::get_character(path.id, authentication.user.id, &db).await {
        Ok(_) => no_content_or_error!(
            bamboo_dbal::character::update_character(
                path.id,
                authentication.user.id,
                body.into_inner(),
                &db
            )
            .await
        ),
        Err(_) => not_found!(bamboo_not_found_error!(
            "character",
            "The character was not found"
        )),
    }
}

pub async fn delete_character(
    path: Option<web::Path<CharacterPathInfo>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    let path = check_invalid_path!(path, "character");

    if !bamboo_dbal::character::character_exists(authentication.user.id, path.id, &db).await {
        return not_found!(bamboo_not_found_error!(
            "character",
            "The character was not found"
        ));
    }

    no_content_or_error!(
        bamboo_dbal::character::delete_character(path.id, authentication.user.id, &db).await
    )
}

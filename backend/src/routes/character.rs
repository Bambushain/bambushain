use actix_web::{HttpResponse, web};
use serde::Deserialize;

use pandaparty_entities::prelude::*;

use crate::DbConnection;
use crate::middleware::authenticate_user::Authentication;

#[derive(Deserialize)]
pub struct CharacterPathInfo {
    pub id: i32,
}

pub async fn get_characters(authentication: Authentication, db: DbConnection) -> HttpResponse {
    ok_or_error!(pandaparty_dbal::character::get_characters(authentication.user.id, &db).await)
}

pub async fn get_character(path: web::Path<CharacterPathInfo>, authentication: Authentication, db: DbConnection) -> HttpResponse {
    match pandaparty_dbal::character::get_character(path.id, authentication.user.id, &db).await {
        Ok(character) => ok_json!(character),
        Err(_) => not_found!(pandaparty_not_found_error!("character", "The character was not found"))
    }
}

pub async fn create_character(body: web::Json<Character>, authentication: Authentication, db: DbConnection) -> HttpResponse {
    if pandaparty_dbal::character::character_exists_by_name(body.name.clone(), authentication.user.id, &db).await {
        return conflict!(pandaparty_exists_already_error!("character", "The character already exists"));
    }

    created_or_error!(pandaparty_dbal::character::create_character(authentication.user.id, body.into_inner(), &db).await)
}

pub async fn update_character(body: web::Json<Character>, path: web::Path<CharacterPathInfo>, authentication: Authentication, db: DbConnection) -> HttpResponse {
    match pandaparty_dbal::character::get_character(path.id, authentication.user.id, &db).await {
        Ok(_) => no_content_or_error!(pandaparty_dbal::character::update_character(path.id, authentication.user.id, body.into_inner(), &db).await),
        Err(_) => not_found!(pandaparty_not_found_error!("character", "The character was not found"))
    }
}

pub async fn delete_character(path: web::Path<CharacterPathInfo>, authentication: Authentication, db: DbConnection) -> HttpResponse {
    if !pandaparty_dbal::character::character_exists(path.id, authentication.user.id, &db).await {
        return not_found!(pandaparty_not_found_error!("character", "The character was not found"));
    }

    no_content_or_error!(pandaparty_dbal::character::delete_character(path.id, authentication.user.id, &db).await)
}

use actix_web::{delete, get, post, put, web};

use bamboo_dbal::prelude::*;
use bamboo_entities::prelude::*;
use bamboo_error::*;
use bamboo_services::prelude::DbConnection;

use crate::middleware::authenticate_user::{authenticate, Authentication};
use crate::middleware::extract_character::{character, CharacterData};
use crate::path;
use crate::response::macros::*;

#[get(
"/api/final-fantasy/character/{character_id}/crafter",
wrap = "authenticate!()",
wrap = "character!()"
)]
pub async fn get_crafters(
    authentication: Authentication,
    character: CharacterData,
    db: DbConnection,
) -> BambooApiResponseResult {
    dbal::get_crafters(authentication.user.id, character.id, &db)
        .await
        .map(|data| list!(data))
}

#[get(
"/api/final-fantasy/character/{character_id}/crafter/{crafter_id}",
wrap = "authenticate!()",
wrap = "character!()"
)]
pub async fn get_crafter(
    path: Option<path::CrafterPath>,
    character: CharacterData,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResult<Crafter> {
    let path = check_invalid_path!(path, "crafter")?;

    dbal::get_crafter(path.crafter_id, authentication.user.id, character.id, &db)
        .await
        .map(|crafter| ok!(crafter))
}

#[post(
"/api/final-fantasy/character/{character_id}/crafter",
wrap = "authenticate!()",
wrap = "character!()"
)]
pub async fn create_crafter(
    body: Option<web::Json<Crafter>>,
    character: CharacterData,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResult<Crafter> {
    let body = check_missing_fields!(body, "crafter")?;

    dbal::create_crafter(authentication.user.id, character.id, body.into_inner(), &db)
        .await
        .map(|data| ok!(data))
}

#[put(
"/api/final-fantasy/character/{character_id}/crafter/{crafter_id}",
wrap = "authenticate!()",
wrap = "character!()"
)]
pub async fn update_crafter(
    body: Option<web::Json<Crafter>>,
    path: Option<path::CrafterPath>,
    character: CharacterData,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "crafter")?;
    let body = check_missing_fields!(body, "crafter")?;

    dbal::update_crafter(
        path.crafter_id,
        authentication.user.id,
        character.id,
        body.into_inner(),
        &db,
    )
        .await
        .map(|_| no_content!())
}

#[delete(
"/api/final-fantasy/character/{character_id}/crafter/{crafter_id}",
wrap = "authenticate!()",
wrap = "character!()"
)]
pub async fn delete_crafter(
    path: Option<path::CrafterPath>,
    character: CharacterData,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "crafter")?;

    dbal::delete_crafter(path.crafter_id, authentication.user.id, character.id, &db)
        .await
        .map(|_| no_content!())
}

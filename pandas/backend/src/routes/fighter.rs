use actix_web::{delete, get, post, put, web};

use bamboo_common::backend::dbal;
use bamboo_common::core::entities::*;
use bamboo_common::core::error::*;
use bamboo_common::backend::services::DbConnection;

use crate::middleware::authenticate_user::{authenticate, Authentication};
use crate::middleware::extract_character::{character, CharacterData};
use crate::path;
use crate::response::macros::*;

#[get(
"/api/final-fantasy/character/{character_id}/fighter",
wrap = "authenticate!()",
wrap = "character!()"
)]
pub async fn get_fighters(
    authentication: Authentication,
    character: CharacterData,
    db: DbConnection,
) -> BambooApiResponseResult {
    dbal::get_fighters(authentication.user.id, character.id, &db)
        .await
        .map(|data| list!(data))
}

#[get(
"/api/final-fantasy/character/{character_id}/fighter/{fighter_id}",
wrap = "authenticate!()",
wrap = "character!()"
)]
pub async fn get_fighter(
    path: Option<path::FighterPath>,
    character: CharacterData,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResult<Fighter> {
    let path = check_invalid_path!(path, "fighter")?;

    dbal::get_fighter(path.fighter_id, authentication.user.id, character.id, &db)
        .await
        .map(|fighter| ok!(fighter))
}

#[post(
"/api/final-fantasy/character/{character_id}/fighter",
wrap = "authenticate!()",
wrap = "character!()"
)]
pub async fn create_fighter(
    body: Option<web::Json<Fighter>>,
    character: CharacterData,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResult<Fighter> {
    let body = check_missing_fields!(body, "fighter")?;

    dbal::create_fighter(authentication.user.id, character.id, body.into_inner(), &db)
        .await
        .map(|data| ok!(data))
}

#[put(
"/api/final-fantasy/character/{character_id}/fighter/{fighter_id}",
wrap = "authenticate!()",
wrap = "character!()"
)]
pub async fn update_fighter(
    body: Option<web::Json<Fighter>>,
    path: Option<path::FighterPath>,
    character: CharacterData,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "fighter")?;
    let body = check_missing_fields!(body, "fighter")?;

    dbal::update_fighter(
        path.fighter_id,
        authentication.user.id,
        character.id,
        body.into_inner(),
        &db,
    )
        .await
        .map(|_| no_content!())
}

#[delete(
"/api/final-fantasy/character/{character_id}/fighter/{fighter_id}",
wrap = "authenticate!()",
wrap = "character!()"
)]
pub async fn delete_fighter(
    path: Option<path::FighterPath>,
    character: CharacterData,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "fighter")?;

    dbal::delete_fighter(path.fighter_id, authentication.user.id, character.id, &db)
        .await
        .map(|_| no_content!())
}

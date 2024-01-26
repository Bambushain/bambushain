use actix_web::{delete, get, post, put, web};

use bamboo_common::backend::dbal;
use bamboo_common::backend::response::*;
use bamboo_common::backend::services::DbConnection;
use bamboo_common::core::entities::*;
use bamboo_common::core::error::*;

use crate::middleware::authenticate_user::{authenticate, Authentication};
use crate::middleware::extract_character::{character, CharacterData};
use crate::path;

#[get(
    "/api/final-fantasy/character/{character_id}/housing",
    wrap = "authenticate!()",
    wrap = "character!()"
)]
pub async fn get_character_housings(
    authentication: Authentication,
    character: CharacterData,
    db: DbConnection,
) -> BambooApiResponseResult {
    dbal::get_character_housings(authentication.user.id, character.id, &db)
        .await
        .map(|data| list!(data))
}

#[get(
    "/api/final-fantasy/character/{character_id}/housing/{character_housing_id}",
    wrap = "authenticate!()",
    wrap = "character!()"
)]
pub async fn get_character_housing(
    path: Option<path::CharacterHousingPath>,
    character: CharacterData,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResult<CharacterHousing> {
    let path = check_invalid_path!(path, "character_housing")?;

    dbal::get_character_housing(
        path.character_housing_id,
        authentication.user.id,
        character.id,
        &db,
    )
    .await
    .map(|character_housing| ok!(character_housing))
}

#[post(
    "/api/final-fantasy/character/{character_id}/housing",
    wrap = "authenticate!()",
    wrap = "character!()"
)]
pub async fn create_character_housing(
    body: Option<web::Json<CharacterHousing>>,
    character: CharacterData,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResult<CharacterHousing> {
    let body = check_missing_fields!(body, "character_housing")?;

    dbal::create_character_housing(authentication.user.id, character.id, body.into_inner(), &db)
        .await
        .map(|data| ok!(data))
}

#[put(
    "/api/final-fantasy/character/{character_id}/housing/{character_housing_id}",
    wrap = "authenticate!()",
    wrap = "character!()"
)]
pub async fn update_character_housing(
    body: Option<web::Json<CharacterHousing>>,
    path: Option<path::CharacterHousingPath>,
    character: CharacterData,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "character_housing")?;
    let body = check_missing_fields!(body, "character_housing")?;

    dbal::update_character_housing(
        path.character_housing_id,
        authentication.user.id,
        character.id,
        body.into_inner(),
        &db,
    )
    .await
    .map(|_| no_content!())
}

#[delete(
    "/api/final-fantasy/character/{character_id}/housing/{character_housing_id}",
    wrap = "authenticate!()",
    wrap = "character!()"
)]
pub async fn delete_character_housing(
    path: Option<path::CharacterHousingPath>,
    character: CharacterData,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "character_housing")?;

    dbal::delete_character_housing(
        path.character_housing_id,
        authentication.user.id,
        character.id,
        &db,
    )
    .await
    .map(|_| no_content!())
}

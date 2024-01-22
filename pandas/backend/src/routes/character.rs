use actix_web::{delete, get, post, put, web};

use bamboo_common::backend::dbal;
use bamboo_common::backend::services::DbConnection;
use bamboo_common::core::entities::*;
use bamboo_common::core::error::*;

use crate::middleware::authenticate_user::{authenticate, Authentication};
use crate::path;
use crate::response::macros::*;

#[get("/api/final-fantasy/character", wrap = "authenticate!()")]
pub async fn get_characters(
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    dbal::get_characters(authentication.user.id, &db)
        .await
        .map(|data| list!(data))
}

#[get(
"/api/final-fantasy/character/{character_id}",
wrap = "authenticate!()"
)]
pub async fn get_character(
    path: Option<path::CharacterPath>,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResult<Character> {
    let path = check_invalid_path!(path, "character")?;

    dbal::get_character(path.character_id, authentication.user.id, &db)
        .await
        .map(|data| ok!(data))
}

#[post("/api/final-fantasy/character", wrap = "authenticate!()")]
pub async fn create_character(
    body: Option<web::Json<Character>>,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResult<Character> {
    let body = check_missing_fields!(body, "character")?;

    dbal::create_character(authentication.user.id, body.into_inner(), &db)
        .await
        .map(|data| created!(data))
}

#[put(
"/api/final-fantasy/character/{character_id}",
wrap = "authenticate!()"
)]
pub async fn update_character(
    body: Option<web::Json<Character>>,
    path: Option<path::CharacterPath>,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "character")?;
    let body = check_missing_fields!(body, "character")?;

    dbal::update_character(
        path.character_id,
        authentication.user.id,
        body.into_inner(),
        &db,
    )
        .await
        .map(|_| no_content!())
}

#[delete(
"/api/final-fantasy/character/{character_id}",
wrap = "authenticate!()"
)]
pub async fn delete_character(
    path: Option<path::CharacterPath>,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "character")?;

    dbal::delete_character(path.character_id, authentication.user.id, &db)
        .await
        .map(|_| no_content!())
}

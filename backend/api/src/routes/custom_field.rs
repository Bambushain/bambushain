use actix_web::{delete, get, post, put, web};

use bamboo_common::backend::dbal;
use bamboo_common::backend::response::*;
use bamboo_common::backend::services::DbConnection;
use bamboo_common::core::entities::*;
use bamboo_common::core::error::*;

use crate::path;
use bamboo_common::backend::actix::middleware::{authenticate, Authentication};

#[get("/api/final-fantasy/character/custom-field", wrap = "authenticate!()")]
pub async fn get_custom_fields(
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    dbal::get_custom_fields(authentication.user.id, &db)
        .await
        .map(|data| list!(data))
}

#[get(
    "/api/final-fantasy/character/custom-field/{field_id}",
    wrap = "authenticate!()"
)]
pub async fn get_custom_field(
    path: Option<path::CustomFieldPath>,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResult<CustomCharacterField> {
    let path = check_invalid_path!(path, "custom_field")?;

    dbal::get_custom_field(path.field_id, authentication.user.id, &db)
        .await
        .map(|data| ok!(data))
}

#[post("/api/final-fantasy/character/custom-field", wrap = "authenticate!()")]
pub async fn create_custom_field(
    body: Option<web::Json<CustomField>>,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResult<CustomCharacterField> {
    let body = check_missing_fields!(body, "custom_field")?;

    dbal::create_custom_field(authentication.user.id, body.into_inner(), &db)
        .await
        .map(|data| created!(data))
}

#[put(
    "/api/final-fantasy/character/custom-field/{field_id}",
    wrap = "authenticate!()"
)]
pub async fn update_custom_field(
    path: Option<path::CustomFieldPath>,
    body: Option<web::Json<CustomField>>,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "custom_field")?;
    let body = check_missing_fields!(body, "custom_field")?;

    dbal::update_custom_field(
        path.field_id,
        authentication.user.id,
        body.into_inner(),
        &db,
    )
    .await
    .map(|_| no_content!())
}

#[delete(
    "/api/final-fantasy/character/custom-field/{field_id}",
    wrap = "authenticate!()"
)]
pub async fn delete_custom_field(
    path: Option<path::CustomFieldPath>,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "custom_field")?;

    dbal::delete_custom_field(path.field_id, authentication.user.id, &db)
        .await
        .map(|_| no_content!())
}

#[post(
    "/api/final-fantasy/character/custom-field/{field_id}/option",
    wrap = "authenticate!()"
)]
pub async fn create_custom_field_option(
    path: Option<path::CustomFieldPath>,
    body: Option<web::Json<String>>,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResult<CustomCharacterFieldOption> {
    let path = check_invalid_path!(path, "custom_field")?;
    let body = check_missing_fields!(body, "custom_field")?;

    dbal::create_custom_field_option(
        authentication.user.id,
        path.field_id,
        body.into_inner(),
        &db,
    )
    .await
    .map(|data| created!(data))
}

#[get(
    "/api/final-fantasy/character/custom-field/{field_id}/option",
    wrap = "authenticate!()"
)]
pub async fn get_custom_field_options(
    path: Option<path::CustomFieldPath>,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "custom_field")?;

    dbal::get_custom_field_options(path.field_id, authentication.user.id, &db)
        .await
        .map(|data| list!(data))
}

#[put(
    "/api/final-fantasy/character/custom-field/{field_id}/option/{option_id}",
    wrap = "authenticate!()"
)]
pub async fn update_custom_field_option(
    path: Option<path::CustomFieldOptionPath>,
    body: Option<web::Json<String>>,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "custom_field")?;
    let body = check_missing_fields!(body, "custom_field")?;

    dbal::update_custom_field_option(
        path.option_id,
        authentication.user.id,
        path.field_id,
        body.into_inner(),
        &db,
    )
    .await
    .map(|_| no_content!())
}

#[delete(
    "/api/final-fantasy/character/custom-field/{field_id}/option/{option_id}",
    wrap = "authenticate!()"
)]
pub async fn delete_custom_field_option(
    path: Option<path::CustomFieldOptionPath>,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "custom_field")?;

    dbal::delete_custom_field_option(path.option_id, path.field_id, &db)
        .await
        .map(|_| no_content!())
}

#[put(
    "/api/final-fantasy/character/custom-field/{field_id}/{position}",
    wrap = "authenticate!()"
)]
pub async fn move_custom_field(
    path: Option<path::CustomFieldPositionPath>,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "custom_field")?;

    dbal::move_custom_field(authentication.user.id, path.field_id, path.position, &db)
        .await
        .map(|_| no_content!())
}

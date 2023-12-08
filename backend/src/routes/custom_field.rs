use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::Deserialize;

use bamboo_entities::prelude::*;
use bamboo_error::*;
use bamboo_services::prelude::DbConnection;

use crate::middleware::authenticate_user::{authenticate, Authentication};

#[derive(Deserialize)]
pub struct CustomFieldPath {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct CustomFieldOptionPath {
    pub id: i32,
    pub field_id: i32,
}

#[derive(Deserialize)]
pub struct CustomFieldPositionPath {
    pub id: i32,
    pub position: i32,
}

#[get("/api/final-fantasy/character/custom-field", wrap = "authenticate!()")]
pub async fn get_custom_fields(authentication: Authentication, db: DbConnection) -> HttpResponse {
    ok_or_error!(bamboo_dbal::custom_field::get_custom_fields(authentication.user.id, &db).await)
}

#[get(
    "/api/final-fantasy/character/custom-field/{id}",
    wrap = "authenticate!()"
)]
pub async fn get_custom_field(
    path: Option<web::Path<CustomFieldPath>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    let path = check_invalid_path!(path, "custom_field");

    match bamboo_dbal::custom_field::get_custom_field(path.id, authentication.user.id, &db).await {
        Ok(custom_field) => ok_json!(custom_field),
        Err(_) => not_found!(bamboo_not_found_error!(
            "custom_field",
            "The custom field was not found"
        )),
    }
}

#[post("/api/final-fantasy/character/custom-field", wrap = "authenticate!()")]
pub async fn create_custom_field(
    body: Option<web::Json<CustomField>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    let body = check_missing_fields!(body, "custom_field");

    if bamboo_dbal::custom_field::custom_field_exists_by_label(
        body.label.clone(),
        authentication.user.id,
        &db,
    )
    .await
    {
        return conflict!(bamboo_exists_already_error!(
            "custom_field",
            "The custom field already exists"
        ));
    }

    created_or_error!(
        bamboo_dbal::custom_field::create_custom_field(
            authentication.user.id,
            body.into_inner(),
            &db
        )
        .await
    )
}

#[put(
    "/api/final-fantasy/character/custom-field/{id}",
    wrap = "authenticate!()"
)]
pub async fn update_custom_field(
    path: Option<web::Path<CustomFieldPath>>,
    body: Option<web::Json<CustomField>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    let path = check_invalid_path!(path, "custom_field");
    let body = check_missing_fields!(body, "custom_field");

    match bamboo_dbal::custom_field::get_custom_field(path.id, authentication.user.id, &db).await {
        Ok(_) => no_content_or_error!(
            bamboo_dbal::custom_field::update_custom_field(
                path.id,
                authentication.user.id,
                body.into_inner(),
                &db
            )
            .await
        ),
        Err(_) => not_found!(bamboo_not_found_error!(
            "custom_field",
            "The custom field was not found"
        )),
    }
}

#[delete(
    "/api/final-fantasy/character/custom-field/{id}",
    wrap = "authenticate!()"
)]
pub async fn delete_custom_field(
    path: Option<web::Path<CustomFieldPath>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    let path = check_invalid_path!(path, "custom_field");

    if !bamboo_dbal::custom_field::custom_field_exists(authentication.user.id, path.id, &db).await {
        return not_found!(bamboo_not_found_error!(
            "custom_field",
            "The custom field was not found"
        ));
    }

    no_content_or_error!(
        bamboo_dbal::custom_field::delete_custom_field(path.id, authentication.user.id, &db).await
    )
}

#[post(
    "/api/final-fantasy/character/custom-field/{id}/option",
    wrap = "authenticate!()"
)]
pub async fn create_custom_field_option(
    path: Option<web::Path<CustomFieldPath>>,
    body: Option<web::Json<String>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    let path = check_invalid_path!(path, "custom_field");
    let body = check_missing_fields!(body, "custom_field");

    if !bamboo_dbal::custom_field::custom_field_exists(authentication.user.id, path.id, &db).await {
        return not_found!(bamboo_not_found_error!(
            "custom_field",
            "The custom field was not found"
        ));
    }

    no_content_or_error!(
        bamboo_dbal::custom_field::create_custom_field_option(path.id, body.into_inner(), &db)
            .await
    )
}

#[get(
    "/api/final-fantasy/character/custom-field/{id}/option",
    wrap = "authenticate!()"
)]
pub async fn get_custom_field_options(
    path: Option<web::Path<CustomFieldPath>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    let path = check_invalid_path!(path, "custom_field");

    if !bamboo_dbal::custom_field::custom_field_exists(authentication.user.id, path.id, &db).await {
        return not_found!(bamboo_not_found_error!(
            "custom_field",
            "The custom field was not found"
        ));
    }

    ok_or_error!(
        bamboo_dbal::custom_field::get_custom_field_options(path.id, authentication.user.id, &db)
            .await
    )
}

#[put(
    "/api/final-fantasy/character/custom-field/{field_id}/option/{id}",
    wrap = "authenticate!()"
)]
pub async fn update_custom_field_option(
    path: Option<web::Path<CustomFieldOptionPath>>,
    body: Option<web::Json<String>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    let path = check_invalid_path!(path, "custom_field");
    let body = check_missing_fields!(body, "custom_field");

    if !bamboo_dbal::custom_field::custom_field_exists(authentication.user.id, path.field_id, &db)
        .await
    {
        return not_found!(bamboo_not_found_error!(
            "custom_field",
            "The custom field was not found"
        ));
    }

    no_content_or_error!(
        bamboo_dbal::custom_field::update_custom_field_option(
            path.id,
            path.field_id,
            body.into_inner(),
            &db
        )
        .await
    )
}

#[delete(
    "/api/final-fantasy/character/custom-field/{field_id}/option/{id}",
    wrap = "authenticate!()"
)]
pub async fn delete_custom_field_option(
    path: Option<web::Path<CustomFieldOptionPath>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    let path = check_invalid_path!(path, "custom_field");

    if !bamboo_dbal::custom_field::custom_field_exists(authentication.user.id, path.field_id, &db)
        .await
    {
        return not_found!(bamboo_not_found_error!(
            "custom_field",
            "The custom field was not found"
        ));
    }

    no_content_or_error!(
        bamboo_dbal::custom_field::delete_custom_field_option(path.id, path.field_id, &db).await
    )
}

#[put(
    "/api/final-fantasy/character/custom-field/{field_id}/{position}",
    wrap = "authenticate!()"
)]
pub async fn move_custom_field(
    path: Option<web::Path<CustomFieldPositionPath>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    let path = check_invalid_path!(path, "custom_field");

    if !bamboo_dbal::custom_field::custom_field_exists(authentication.user.id, path.id, &db).await {
        return not_found!(bamboo_not_found_error!(
            "custom_field",
            "The custom field was not found"
        ));
    }

    no_content_or_error!(
        bamboo_dbal::custom_field::move_custom_field(
            authentication.user.id,
            path.id,
            path.position,
            &db
        )
        .await
    )
}

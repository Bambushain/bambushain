use actix_web::{web, HttpResponse};
use serde::Deserialize;

use pandaparty_entities::prelude::*;

use crate::middleware::authenticate_user::Authentication;
use crate::DbConnection;

#[derive(Deserialize)]
pub struct CustomFieldPath {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct CustomFieldOptionPath {
    pub id: i32,
    pub field_id: i32,
}

pub async fn get_custom_fields(authentication: Authentication, db: DbConnection) -> HttpResponse {
    ok_or_error!(
        pandaparty_dbal::custom_field::get_custom_fields(authentication.user.id, &db).await
    )
}

pub async fn get_custom_field(
    path: web::Path<CustomFieldPath>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    match pandaparty_dbal::custom_field::get_custom_field(path.id, authentication.user.id, &db)
        .await
    {
        Ok(custom_field) => ok_json!(custom_field),
        Err(_) => not_found!(pandaparty_not_found_error!(
            "custom_field",
            "The custom field was not found"
        )),
    }
}

pub async fn create_custom_field(
    body: web::Json<CustomField>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    if pandaparty_dbal::custom_field::custom_field_exists_by_label(
        body.label.clone(),
        authentication.user.id,
        &db,
    )
    .await
    {
        return conflict!(pandaparty_exists_already_error!(
            "custom_field",
            "The custom field already exists"
        ));
    }

    created_or_error!(
        pandaparty_dbal::custom_field::create_custom_field(
            authentication.user.id,
            body.into_inner(),
            &db
        )
        .await
    )
}

pub async fn update_custom_field(
    path: web::Path<CustomFieldPath>,
    body: web::Json<CustomField>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    match pandaparty_dbal::custom_field::get_custom_field(path.id, authentication.user.id, &db)
        .await
    {
        Ok(_) => no_content_or_error!(
            pandaparty_dbal::custom_field::update_custom_field(
                path.id,
                authentication.user.id,
                body.into_inner(),
                &db
            )
            .await
        ),
        Err(_) => not_found!(pandaparty_not_found_error!(
            "custom_field",
            "The custom field was not found"
        )),
    }
}

pub async fn delete_custom_field(
    path: web::Path<CustomFieldPath>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    if !pandaparty_dbal::custom_field::custom_field_exists(authentication.user.id, path.id, &db)
        .await
    {
        return not_found!(pandaparty_not_found_error!(
            "custom_field",
            "The custom field was not found"
        ));
    }

    no_content_or_error!(
        pandaparty_dbal::custom_field::delete_custom_field(path.id, authentication.user.id, &db)
            .await
    )
}

pub async fn create_custom_field_option(
    path: web::Path<CustomFieldPath>,
    body: web::Json<String>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    if !pandaparty_dbal::custom_field::custom_field_exists(authentication.user.id, path.id, &db)
        .await
    {
        return not_found!(pandaparty_not_found_error!(
            "custom_field",
            "The custom field was not found"
        ));
    }

    no_content_or_error!(
        pandaparty_dbal::custom_field::create_custom_field_option(path.id, body.into_inner(), &db)
            .await
    )
}

pub async fn get_custom_field_options(
    path: web::Path<CustomFieldPath>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    if !pandaparty_dbal::custom_field::custom_field_exists(authentication.user.id, path.id, &db)
        .await
    {
        return not_found!(pandaparty_not_found_error!(
            "custom_field",
            "The custom field was not found"
        ));
    }

    ok_or_error!(
        pandaparty_dbal::custom_field::get_custom_field_options(
            path.id,
            authentication.user.id,
            &db
        )
        .await
    )
}

pub async fn update_custom_field_option(
    path: web::Path<CustomFieldOptionPath>,
    body: web::Json<String>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    if !pandaparty_dbal::custom_field::custom_field_exists(
        authentication.user.id,
        path.field_id,
        &db,
    )
    .await
    {
        return not_found!(pandaparty_not_found_error!(
            "custom_field",
            "The custom field was not found"
        ));
    }

    no_content_or_error!(
        pandaparty_dbal::custom_field::update_custom_field_option(
            path.id,
            path.field_id,
            body.into_inner(),
            &db
        )
        .await
    )
}

pub async fn delete_custom_field_option(
    path: web::Path<CustomFieldOptionPath>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    if !pandaparty_dbal::custom_field::custom_field_exists(
        authentication.user.id,
        path.field_id,
        &db,
    )
    .await
    {
        return not_found!(pandaparty_not_found_error!(
            "custom_field",
            "The custom field was not found"
        ));
    }

    no_content_or_error!(
        pandaparty_dbal::custom_field::delete_custom_field_option(path.id, path.field_id, &db)
            .await
    )
}

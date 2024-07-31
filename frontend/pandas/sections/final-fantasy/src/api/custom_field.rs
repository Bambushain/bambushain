use std::collections::BTreeSet;

use bamboo_common::core::entities::{
    CustomCharacterField, CustomCharacterFieldOption, CustomField,
};
use bamboo_common::frontend::api::BambooApiResult;
use bamboo_frontend_pandas_base::api::{delete, get, post, put_no_body_no_content, put_no_content};

pub async fn get_custom_fields() -> BambooApiResult<Vec<CustomCharacterField>> {
    log::debug!("Get custom fields");
    get("/api/final-fantasy/character/custom-field").await
}

pub async fn create_custom_field(
    label: String,
    position: usize,
) -> BambooApiResult<CustomCharacterField> {
    log::debug!("Create new field: {label}");
    post(
        "/api/final-fantasy/character/custom-field",
        &CustomField {
            label,
            values: BTreeSet::new(),
            position,
        },
    )
    .await
}

pub async fn update_custom_field(id: i32, label: String) -> BambooApiResult<()> {
    log::debug!("Update field: {id} {label}");
    put_no_content(
        format!("/api/final-fantasy/character/custom-field/{id}"),
        &CustomField {
            label,
            values: BTreeSet::new(),
            ..Default::default()
        },
    )
    .await
}

pub async fn delete_custom_field(id: i32) -> BambooApiResult<()> {
    log::debug!("Delete field: {id}");
    delete(format!("/api/final-fantasy/character/custom-field/{id}")).await
}

pub async fn add_custom_field_option(
    field_id: i32,
    label: String,
) -> BambooApiResult<CustomCharacterFieldOption> {
    log::debug!("Create field option: {field_id} {label}");
    post(
        format!("/api/final-fantasy/character/custom-field/{field_id}/option"),
        &label,
    )
    .await
}

pub async fn update_custom_field_option(
    field_id: i32,
    id: i32,
    label: String,
) -> BambooApiResult<()> {
    log::debug!("Rename field option: {field_id} {id} {label}");
    put_no_content(
        format!("/api/final-fantasy/character/custom-field/{field_id}/option/{id}"),
        &label,
    )
    .await
}

pub async fn delete_custom_field_option(field_id: i32, id: i32) -> BambooApiResult<()> {
    log::debug!("Delete field option: {field_id} {id}");
    delete(format!(
        "/api/final-fantasy/character/custom-field/{field_id}/option/{id}"
    ))
    .await
}

pub async fn get_custom_field_options(
    field_id: i32,
) -> BambooApiResult<Vec<CustomCharacterFieldOption>> {
    log::debug!("Get custom field options for field {field_id}");
    get(format!(
        "/api/final-fantasy/character/custom-field/{field_id}/option"
    ))
    .await
}

pub async fn move_custom_field(id: i32, position: i32) -> BambooApiResult<()> {
    log::debug!("Move field: {id} {position}");
    put_no_body_no_content(format!(
        "/api/final-fantasy/character/custom-field/{id}/{position}"
    ))
    .await
}

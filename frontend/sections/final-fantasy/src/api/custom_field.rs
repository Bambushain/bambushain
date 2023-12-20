use std::collections::BTreeSet;
use std::rc::Rc;

use async_trait::async_trait;
use bounce::query::{Query, QueryResult};
use bounce::BounceStates;

use bamboo_entities::prelude::{CustomCharacterField, CustomField};
use bamboo_frontend_base_api::{
    delete, get, post, post_no_content, put_no_body_no_content, put_no_content, ApiError,
    BambooApiResult,
};

use crate::models::CustomCharacterFields;

pub async fn get_custom_fields() -> BambooApiResult<Vec<CustomCharacterField>> {
    log::debug!("Get custom fields");
    get("/api/final-fantasy/character/custom-field").await
}

#[async_trait(? Send)]
impl Query for CustomCharacterFields {
    type Input = ();
    type Error = ApiError;

    async fn query(_states: &BounceStates, _input: Rc<Self::Input>) -> QueryResult<Self> {
        get_custom_fields()
            .await
            .map(|fields| Rc::new(fields.into()))
    }
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

pub async fn update_custom_field(id: i32, label: String, position: usize) -> BambooApiResult<()> {
    log::debug!("Update field: {id} {label}");
    put_no_content(
        format!("/api/final-fantasy/character/custom-field/{id}"),
        &CustomField {
            label,
            values: BTreeSet::new(),
            position,
        },
    )
    .await
}

pub async fn delete_custom_field(id: i32) -> BambooApiResult<()> {
    log::debug!("Delete field: {id}");
    delete(format!("/api/final-fantasy/character/custom-field/{id}")).await
}

pub async fn add_custom_field_option(field_id: i32, label: String) -> BambooApiResult<()> {
    log::debug!("Create field option: {field_id} {label}");
    post_no_content(
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

pub async fn move_custom_field(id: i32, position: usize) -> BambooApiResult<()> {
    log::debug!("Move field: {id} {position}");
    put_no_body_no_content(format!(
        "/api/final-fantasy/character/custom-field/{id}/{position}"
    ))
    .await
}
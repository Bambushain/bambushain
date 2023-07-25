use std::rc::Rc;

use async_trait::async_trait;
use bounce::BounceStates;
use bounce::query::{Query, QueryResult};

use sheef_entities::prelude::*;

use crate::api::{ApiError, delete, post, put, put_no_body, SheefApiResult};
use crate::api::boolean_table::{BooleanTable, get_boolean_table};

async fn get_savage_mounts() -> SheefApiResult<BooleanTable> {
    log::debug!("Get savage mounts");
    get_boolean_table("/api/savage-mount".to_string()).await
}

#[derive(PartialEq, Clone, Default)]
pub struct SavageMounts {
    pub data: BooleanTable,
}

#[async_trait(? Send)]
impl Query for SavageMounts {
    type Input = ();
    type Error = ApiError;

    async fn query(_states: &BounceStates, _input: Rc<Self::Input>) -> QueryResult<Self> {
        get_savage_mounts().await.map(|savage_mounts| Rc::new(SavageMounts { data: savage_mounts }))
    }
}

pub async fn activate_savage_mount(user: String, savage_mount: String) -> SheefApiResult<()> {
    log::debug!("Activate savage mount {savage_mount} for user {user}");
    put_no_body(format!("/api/user/{user}/savage-mount/{savage_mount}")).await
}

pub async fn deactivate_savage_mount(user: String, savage_mount: String) -> SheefApiResult<()> {
    log::debug!("Deactivate savage mount {savage_mount} for user {user}");
    delete(format!("/api/user/{user}/savage-mount/{savage_mount}")).await
}

pub async fn delete_savage_mount(savage_mount: String) -> SheefApiResult<()> {
    log::debug!("Delete savage mount {savage_mount}");
    delete(format!("/api/savage-mount/{savage_mount}")).await
}

pub async fn create_savage_mount(name: String) -> SheefApiResult<SavageMount> {
    log::debug!("Create new savage mount {name}");
    post("/api/savage-mount", &SavageMount { name }).await
}

pub async fn rename_savage_mount(old_name: String, new_name: String) -> SheefApiResult<()> {
    log::debug!("Rename savage mount {old_name} to {new_name}");
    put(format!("/api/savage-mount/{old_name}"), &SavageMount { name: new_name }).await
}
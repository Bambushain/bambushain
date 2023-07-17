use std::rc::Rc;
use async_trait::async_trait;
use bounce::BounceStates;
use bounce::query::{Query, QueryResult};
use crate::api::boolean_table::{BooleanTable, get_boolean_table};
use crate::api::{delete, ErrorCode, post, put, put_no_body};

async fn get_savage_mounts() -> Result<BooleanTable, ErrorCode> {
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
    type Error = ErrorCode;

    async fn query(_states: &BounceStates, _input: Rc<Self::Input>) -> QueryResult<Self> {
        get_savage_mounts().await.map(|savage_mounts| Rc::new(SavageMounts { data: savage_mounts }))
    }
}

pub async fn activate_savage_mount(user: String, savage_mount: String) -> ErrorCode {
    log::debug!("Activate savage mount {savage_mount} for user {user}");
    put_no_body(format!("/api/user/{user}/savage-mount/{savage_mount}")).await
}

pub async fn deactivate_savage_mount(user: String, savage_mount: String) -> ErrorCode {
    log::debug!("Deactivate savage mount {savage_mount} for user {user}");
    delete(format!("/api/user/{user}/savage-mount/{savage_mount}")).await
}

pub async fn delete_savage_mount(savage_mount: String) -> ErrorCode {
    log::debug!("Delete savage mount {savage_mount}");
    delete(format!("/api/savage-mount/{savage_mount}")).await
}

pub async fn create_savage_mount(name: String) -> Result<sheef_entities::SavageMount, ErrorCode> {
    log::debug!("Create new savage mount {name}");
    post("/api/savage-mount", &sheef_entities::SavageMount { name }).await
}

pub async fn rename_savage_mount(old_name: String, new_name: String) -> ErrorCode {
    log::debug!("Rename savage mount {old_name} to {new_name}");
    put(format!("/api/savage-mount/{old_name}"), &sheef_entities::SavageMount { name: new_name }).await
}
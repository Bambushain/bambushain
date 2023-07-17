use std::rc::Rc;
use async_trait::async_trait;
use bounce::BounceStates;
use bounce::query::{Query, QueryResult};
use crate::api::boolean_table::{BooleanTable, get_boolean_table};
use crate::api::{delete, ErrorCode, post, put, put_no_body};

async fn get_mounts() -> Result<BooleanTable, ErrorCode> {
    log::debug!("Get mounts");
    get_boolean_table("/api/mount".to_string()).await
}

#[derive(PartialEq, Clone, Default)]
pub struct Mounts {
    pub data: BooleanTable,
}

#[async_trait(? Send)]
impl Query for Mounts {
    type Input = ();
    type Error = ErrorCode;

    async fn query(_states: &BounceStates, _input: Rc<Self::Input>) -> QueryResult<Self> {
        get_mounts().await.map(|mounts| Rc::new(Mounts { data: mounts }))
    }
}

pub async fn activate_mount(user: String, mount: String) -> ErrorCode {
    log::debug!("Activate mount {mount} for user {user}");
    put_no_body(format!("/api/user/{user}/mount/{mount}")).await
}

pub async fn deactivate_mount(user: String, mount: String) -> ErrorCode {
    log::debug!("Deactivate mount {mount} for user {user}");
    delete(format!("/api/user/{user}/mount/{mount}")).await
}

pub async fn delete_mount(mount: String) -> ErrorCode {
    log::debug!("Delete mount {mount}");
    delete(format!("/api/mount/{mount}")).await
}

pub async fn create_mount(name: String) -> Result<sheef_entities::Mount, ErrorCode> {
    log::debug!("Create new mount {name}");
    post("/api/mount", &sheef_entities::Mount { name }).await
}

pub async fn rename_mount(old_name: String, new_name: String) -> ErrorCode {
    log::debug!("Rename mount {old_name} to {new_name}");
    put(format!("/api/mount/{old_name}"), &sheef_entities::Mount { name: new_name }).await
}
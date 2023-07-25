use std::rc::Rc;

use async_trait::async_trait;
use bounce::BounceStates;
use bounce::query::{Query, QueryResult};

use sheef_entities::prelude::*;

use crate::api::{ApiError, delete, post, put, put_no_body, SheefApiResult};
use crate::api::boolean_table::{BooleanTable, get_boolean_table};

async fn get_mounts() -> SheefApiResult<BooleanTable> {
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
    type Error = ApiError;

    async fn query(_states: &BounceStates, _input: Rc<Self::Input>) -> QueryResult<Self> {
        get_mounts().await.map(|mounts| Rc::new(Mounts { data: mounts }))
    }
}

pub async fn activate_mount(user: String, mount: String) -> SheefApiResult<()> {
    log::debug!("Activate mount {mount} for user {user}");
    put_no_body(format!("/api/user/{user}/mount/{mount}")).await
}

pub async fn deactivate_mount(user: String, mount: String) -> SheefApiResult<()> {
    log::debug!("Deactivate mount {mount} for user {user}");
    delete(format!("/api/user/{user}/mount/{mount}")).await
}

pub async fn delete_mount(mount: String) -> SheefApiResult<()> {
    log::debug!("Delete mount {mount}");
    delete(format!("/api/mount/{mount}")).await
}

pub async fn create_mount(name: String) -> SheefApiResult<Mount> {
    log::debug!("Create new mount {name}");
    post("/api/mount", &Mount { id: 0, name }).await
}

pub async fn rename_mount(old_name: String, new_name: String) -> SheefApiResult<()> {
    log::debug!("Rename mount {old_name} to {new_name}");
    put(format!("/api/mount/{old_name}"), &Mount { id: 0, name: new_name }).await
}
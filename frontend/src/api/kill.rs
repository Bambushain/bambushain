use std::rc::Rc;

use async_trait::async_trait;
use bounce::BounceStates;
use bounce::query::{Query, QueryResult};

use sheef_entities::prelude::*;

use crate::api::{ApiError, delete, post, put, put_no_body, SheefApiResult};
use crate::api::boolean_table::{BooleanTable, get_boolean_table};

async fn get_kills() -> SheefApiResult<BooleanTable> {
    log::debug!("Get kills");
    get_boolean_table("/api/kill".to_string()).await
}

#[derive(PartialEq, Clone, Default)]
pub struct Kills {
    pub data: BooleanTable,
}

#[async_trait(? Send)]
impl Query for Kills {
    type Input = ();
    type Error = ApiError;

    async fn query(_states: &BounceStates, _input: Rc<Self::Input>) -> QueryResult<Self> {
        get_kills().await.map(|kills| Rc::new(Kills { data: kills }))
    }
}

pub async fn activate_kill(user: String, kill: String) -> SheefApiResult<()> {
    log::debug!("Activate kill {kill} for user {user}");
    put_no_body(format!("/api/user/{user}/kill/{kill}")).await
}

pub async fn deactivate_kill(user: String, kill: String) -> SheefApiResult<()> {
    log::debug!("Deactivate kill {kill} for user {user}");
    delete(format!("/api/user/{user}/kill/{kill}")).await
}

pub async fn delete_kill(kill: String) -> SheefApiResult<()> {
    log::debug!("Delete kill {kill}");
    delete(format!("/api/kill/{kill}")).await
}

pub async fn create_kill(name: String) -> SheefApiResult<Kill> {
    log::debug!("Create new kill {name}");
    post("/api/kill", &Kill { name }).await
}

pub async fn rename_kill(old_name: String, new_name: String) -> SheefApiResult<()> {
    log::debug!("Rename kill {old_name} to {new_name}");
    put(format!("/api/kill/{old_name}"), &Kill { name: new_name }).await
}
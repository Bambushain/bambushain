use std::rc::Rc;
use async_trait::async_trait;
use bounce::BounceStates;
use bounce::query::{Query, QueryResult};
use crate::api::boolean_table::{BooleanTable, get_boolean_table};
use crate::api::{delete, ErrorCode, post, put, put_no_body};

async fn get_kills() -> Result<BooleanTable, ErrorCode> {
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
    type Error = ErrorCode;

    async fn query(_states: &BounceStates, _input: Rc<Self::Input>) -> QueryResult<Self> {
        get_kills().await.map(|kills| Rc::new(Kills { data: kills }))
    }
}

pub async fn activate_kill(user: String, kill: String) -> ErrorCode {
    log::debug!("Activate kill {kill} for user {user}");
    put_no_body(format!("/api/user/{user}/kill/{kill}")).await
}

pub async fn deactivate_kill(user: String, kill: String) -> ErrorCode {
    log::debug!("Deactivate kill {kill} for user {user}");
    delete(format!("/api/user/{user}/kill/{kill}")).await
}

pub async fn delete_kill(kill: String) -> ErrorCode {
    log::debug!("Delete kill {kill}");
    delete(format!("/api/kill/{kill}")).await
}

pub async fn create_kill(name: String) -> Result<sheef_entities::Kill, ErrorCode> {
    log::debug!("Create new kill {name}");
    post("/api/kill", &sheef_entities::Kill { name }).await
}

pub async fn rename_kill(old_name: String, new_name: String) -> ErrorCode {
    log::debug!("Rename kill {old_name} to {new_name}");
    put(format!("/api/kill/{old_name}"), &sheef_entities::Kill { name: new_name }).await
}
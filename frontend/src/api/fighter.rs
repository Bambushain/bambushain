use std::rc::Rc;

use async_trait::async_trait;
use bounce::BounceStates;
use bounce::query::{Query, QueryResult};

use pandaparty_entities::prelude::*;

use crate::api::{ApiError, delete, get, post, put, SheefApiResult};

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct MyFighter {
    pub fighter: Vec<Fighter>,
}

impl From<Vec<Fighter>> for MyFighter {
    fn from(value: Vec<Fighter>) -> Self {
        Self {
            fighter: value,
        }
    }
}

async fn get_fighter() -> SheefApiResult<Vec<Fighter>> {
    log::debug!("Get fighter");
    get("/api/fighter").await
}

#[async_trait(? Send)]
impl Query for MyFighter {
    type Input = ();
    type Error = ApiError;

    async fn query(_states: &BounceStates, _input: Rc<Self::Input>) -> QueryResult<Self> {
        get_fighter().await.map(|fighter| Rc::new(fighter.into()))
    }
}

pub async fn create_fighter(fighter: Fighter) -> SheefApiResult<Fighter> {
    log::debug!("Create fighter {}", fighter.job.clone().get_job_name());
    post("/api/fighter", &fighter).await
}

pub async fn update_fighter(id: i32, fighter: Fighter) -> SheefApiResult<()> {
    log::debug!("Update fighter {id}");
    put(format!("/api/fighter/{id}"), &fighter).await
}

pub async fn delete_fighter(id: i32) -> SheefApiResult<()> {
    log::debug!("Delete fighter {id}");
    delete(format!("/api/fighter/{id}")).await
}
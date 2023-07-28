use std::rc::Rc;

use async_trait::async_trait;
use bounce::BounceStates;
use bounce::query::{Query, QueryResult};

use sheef_entities::prelude::*;

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
    log::debug!("Create fighter {}", fighter.job);
    post("/api/fighter", &fighter).await
}

pub async fn update_fighter(job: String, fighter: Fighter) -> SheefApiResult<()> {
    log::debug!("Create fighter {}", fighter.job);
    put(format!("/api/fighter/{}", job), &fighter).await
}

pub async fn delete_fighter(fighter: Fighter) -> SheefApiResult<()> {
    log::debug!("Delete fighter {}", fighter.job);
    delete(format!("/api/fighter/{}", fighter.job)).await
}
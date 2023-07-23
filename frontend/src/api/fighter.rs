use std::rc::Rc;

use async_trait::async_trait;
use bounce::BounceStates;
use bounce::query::{Query, QueryResult};

use crate::api::{ApiError, delete, get, post, put, SheefApiResult};

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct MyFighter {
    pub fighter: Vec<sheef_entities::Fighter>,
}

impl From<Vec<sheef_entities::Fighter>> for MyFighter {
    fn from(value: Vec<sheef_entities::Fighter>) -> Self {
        Self {
            fighter: value,
        }
    }
}

async fn get_fighter() -> SheefApiResult<Vec<sheef_entities::Fighter>> {
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

pub async fn create_fighter(fighter: sheef_entities::Fighter) -> SheefApiResult<sheef_entities::Fighter> {
    log::debug!("Create fighter {}", fighter.job);
    post("/api/fighter", &fighter).await
}

pub async fn update_fighter(job: String, fighter: sheef_entities::Fighter) -> SheefApiResult<()> {
    log::debug!("Create fighter {}", fighter.job);
    put(format!("/api/fighter/{}", job), &fighter).await
}

pub async fn delete_fighter(fighter: sheef_entities::Fighter) -> SheefApiResult<()> {
    log::debug!("Delete fighter {}", fighter.job);
    delete(format!("/api/fighter/{}", fighter.job)).await
}
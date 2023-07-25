use std::rc::Rc;

use async_trait::async_trait;
use bounce::BounceStates;
use bounce::query::{Query, QueryResult};

use sheef_entities::prelude::*;

use crate::api::{ApiError, delete, get, post, put, SheefApiResult};

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct MyCrafter {
    pub crafter: Vec<Crafter>,
}

impl From<Vec<Crafter>> for MyCrafter {
    fn from(value: Vec<Crafter>) -> Self {
        Self {
            crafter: value,
        }
    }
}

async fn get_crafter() -> SheefApiResult<Vec<Crafter>> {
    log::debug!("Get crafter");
    get("/api/crafter").await
}

#[async_trait(? Send)]
impl Query for MyCrafter {
    type Input = ();
    type Error = ApiError;

    async fn query(_states: &BounceStates, _input: Rc<Self::Input>) -> QueryResult<Self> {
        get_crafter().await.map(|crafter| Rc::new(crafter.into()))
    }
}

pub async fn create_crafter(crafter: Crafter) -> SheefApiResult<Crafter> {
    log::debug!("Create crafter {}", crafter.job);
    post("/api/crafter", &crafter).await
}

pub async fn update_crafter(job: String, crafter: Crafter) -> SheefApiResult<()> {
    log::debug!("Create crafter {}", crafter.job);
    put(format!("/api/crafter/{}", job), &crafter).await
}

pub async fn delete_crafter(crafter: Crafter) -> SheefApiResult<()> {
    log::debug!("Delete crafter {}", crafter.job);
    delete(format!("/api/crafter/{}", crafter.job)).await
}
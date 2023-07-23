use std::rc::Rc;

use async_trait::async_trait;
use bounce::BounceStates;
use bounce::query::{Query, QueryResult};

use crate::api::{ApiError, delete, get, post, put, SheefApiResult};

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct MyCrafter {
    pub crafter: Vec<sheef_entities::Crafter>,
}

impl From<Vec<sheef_entities::Crafter>> for MyCrafter {
    fn from(value: Vec<sheef_entities::Crafter>) -> Self {
        Self {
            crafter: value,
        }
    }
}

async fn get_crafter() -> SheefApiResult<Vec<sheef_entities::Crafter>> {
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

pub async fn create_crafter(crafter: sheef_entities::Crafter) -> SheefApiResult<sheef_entities::Crafter> {
    log::debug!("Create crafter {}", crafter.job);
    post("/api/crafter", &crafter).await
}

pub async fn update_crafter(job: String, crafter: sheef_entities::Crafter) -> SheefApiResult<()> {
    log::debug!("Create crafter {}", crafter.job);
    put(format!("/api/crafter/{}", job), &crafter).await
}

pub async fn delete_crafter(crafter: sheef_entities::Crafter) -> SheefApiResult<()> {
    log::debug!("Delete crafter {}", crafter.job);
    delete(format!("/api/crafter/{}", crafter.job)).await
}
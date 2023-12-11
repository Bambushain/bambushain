use std::rc::Rc;

use async_trait::async_trait;
use bounce::query::{Query, QueryResult};
use bounce::BounceStates;

use bamboo_entities::prelude::*;

use crate::api::{delete, get, post, put_no_content, ApiError, BambooApiResult};

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct CrafterForCharacter {
    pub crafter: Vec<Crafter>,
}

impl From<Vec<Crafter>> for CrafterForCharacter {
    fn from(value: Vec<Crafter>) -> Self {
        Self { crafter: value }
    }
}

async fn get_crafter(character_id: i32) -> BambooApiResult<Vec<Crafter>> {
    log::debug!("Get crafter");
    get(format!(
        "/api/final-fantasy/character/{character_id}/crafter"
    ))
    .await
}

#[async_trait(? Send)]
impl Query for CrafterForCharacter {
    type Input = i32;
    type Error = ApiError;

    async fn query(_states: &BounceStates, input: Rc<Self::Input>) -> QueryResult<Self> {
        get_crafter(*input)
            .await
            .map(|crafter| Rc::new(crafter.into()))
    }
}

pub async fn create_crafter(character_id: i32, crafter: Crafter) -> BambooApiResult<Crafter> {
    log::debug!("Create crafter {}", crafter.job.get_job_name());
    post(
        format!("/api/final-fantasy/character/{character_id}/crafter"),
        &crafter,
    )
    .await
}

pub async fn update_crafter(character_id: i32, id: i32, crafter: Crafter) -> BambooApiResult<()> {
    log::debug!("Update crafter {id}");
    put_no_content(
        format!("/api/final-fantasy/character/{character_id}/crafter/{id}"),
        &crafter,
    )
    .await
}

pub async fn delete_crafter(character_id: i32, id: i32) -> BambooApiResult<()> {
    log::debug!("Delete crafter {id}");
    delete(format!(
        "/api/final-fantasy/character/{character_id}/crafter/{id}"
    ))
    .await
}

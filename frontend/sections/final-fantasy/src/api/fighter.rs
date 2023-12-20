use std::rc::Rc;

use async_trait::async_trait;
use bounce::query::{Query, QueryResult};
use bounce::BounceStates;

use bamboo_entities::prelude::*;
use bamboo_frontend_base_api::{delete, get, post, put_no_content, ApiError, BambooApiResult};

use crate::models::FighterForCharacter;

async fn get_fighter(character_id: i32) -> BambooApiResult<Vec<Fighter>> {
    log::debug!("Get fighter");
    get(format!(
        "/api/final-fantasy/character/{character_id}/fighter"
    ))
    .await
}

#[async_trait(? Send)]
impl Query for FighterForCharacter {
    type Input = i32;
    type Error = ApiError;

    async fn query(_states: &BounceStates, input: Rc<Self::Input>) -> QueryResult<Self> {
        get_fighter(*input)
            .await
            .map(|fighter| Rc::new(fighter.into()))
    }
}

pub async fn create_fighter(character_id: i32, fighter: Fighter) -> BambooApiResult<Fighter> {
    log::debug!("Create fighter {}", fighter.job.get_job_name());
    post(
        format!("/api/final-fantasy/character/{character_id}/fighter"),
        &fighter,
    )
    .await
}

pub async fn update_fighter(character_id: i32, id: i32, fighter: Fighter) -> BambooApiResult<()> {
    log::debug!("Update fighter {id}");
    put_no_content(
        format!("/api/final-fantasy/character/{character_id}/fighter/{id}"),
        &fighter,
    )
    .await
}

pub async fn delete_fighter(character_id: i32, id: i32) -> BambooApiResult<()> {
    log::debug!("Delete fighter {id}");
    delete(format!(
        "/api/final-fantasy/character/{character_id}/fighter/{id}"
    ))
    .await
}

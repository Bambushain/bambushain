use std::rc::Rc;

use async_trait::async_trait;
use bounce::query::{Query, QueryResult};
use bounce::BounceStates;

use pandaparty_entities::prelude::*;

use crate::api::{delete, get, post, put_no_content, ApiError, PandapartyApiResult};

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct FighterForCharacter {
    pub fighter: Vec<Fighter>,
}

impl From<Vec<Fighter>> for FighterForCharacter {
    fn from(value: Vec<Fighter>) -> Self {
        Self { fighter: value }
    }
}

async fn get_fighter(character_id: i32) -> PandapartyApiResult<Vec<Fighter>> {
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

pub async fn create_fighter(character_id: i32, fighter: Fighter) -> PandapartyApiResult<Fighter> {
    log::debug!("Create fighter {}", fighter.job.get_job_name());
    post(
        format!("/api/final-fantasy/character/{character_id}/fighter"),
        &fighter,
    )
    .await
}

pub async fn update_fighter(
    character_id: i32,
    id: i32,
    fighter: Fighter,
) -> PandapartyApiResult<()> {
    log::debug!("Update fighter {id}");
    put_no_content(
        format!("/api/final-fantasy/character/{character_id}/fighter/{id}"),
        &fighter,
    )
    .await
}

pub async fn delete_fighter(character_id: i32, id: i32) -> PandapartyApiResult<()> {
    log::debug!("Delete fighter {id}");
    delete(format!(
        "/api/final-fantasy/character/{character_id}/fighter/{id}"
    ))
    .await
}

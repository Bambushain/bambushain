use std::rc::Rc;

use async_trait::async_trait;
use bounce::query::{Query, QueryResult};
use bounce::BounceStates;

use bamboo_entities::prelude::*;
use bamboo_frontend_base_api::{delete, get, post, put_no_content, ApiError, BambooApiResult};

use crate::models::CharacterHousingForCharacter;

async fn get_character_housing(character_id: i32) -> BambooApiResult<Vec<CharacterHousing>> {
    log::debug!("Get character housing");
    get(format!(
        "/api/final-fantasy/character/{character_id}/housing"
    ))
    .await
}

#[async_trait(? Send)]
impl Query for CharacterHousingForCharacter {
    type Input = i32;
    type Error = ApiError;

    async fn query(_states: &BounceStates, input: Rc<Self::Input>) -> QueryResult<Self> {
        get_character_housing(*input)
            .await
            .map(|character_housing| Rc::new(character_housing.into()))
    }
}

pub async fn create_character_housing(
    character_id: i32,
    character_housing: CharacterHousing,
) -> BambooApiResult<CharacterHousing> {
    log::debug!("Create character housing");
    post(
        format!("/api/final-fantasy/character/{character_id}/housing"),
        &character_housing,
    )
    .await
}

pub async fn update_character_housing(
    character_id: i32,
    id: i32,
    character_housing: CharacterHousing,
) -> BambooApiResult<()> {
    log::debug!("Update character housing {id}");
    put_no_content(
        format!("/api/final-fantasy/character/{character_id}/housing/{id}"),
        &character_housing,
    )
    .await
}

pub async fn delete_character_housing(character_id: i32, id: i32) -> BambooApiResult<()> {
    log::debug!("Delete character housing {id}");
    delete(format!(
        "/api/final-fantasy/character/{character_id}/housing/{id}"
    ))
    .await
}

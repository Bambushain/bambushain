use std::rc::Rc;

use async_trait::async_trait;
use bounce::query::{Query, QueryResult};
use bounce::BounceStates;

use bamboo_entities::prelude::*;
use bamboo_frontend_base_api::{delete, get, post, put_no_content, ApiError, BambooApiResult};

use crate::models::MyCharacters;

async fn get_character() -> BambooApiResult<Vec<Character>> {
    log::debug!("Get character");
    get("/api/final-fantasy/character").await
}

#[async_trait(? Send)]
impl Query for MyCharacters {
    type Input = ();
    type Error = ApiError;

    async fn query(_states: &BounceStates, _input: Rc<Self::Input>) -> QueryResult<Self> {
        get_character()
            .await
            .map(|character| Rc::new(character.into()))
    }
}

pub async fn create_character(character: Character) -> BambooApiResult<Character> {
    log::debug!("Create character {}", character.name);
    post("/api/final-fantasy/character", &character).await
}

pub async fn update_character(id: i32, character: Character) -> BambooApiResult<()> {
    log::debug!("Update character {id}");
    put_no_content(format!("/api/final-fantasy/character/{id}"), &character).await
}

pub async fn delete_character(id: i32) -> BambooApiResult<()> {
    log::debug!("Delete character {id}");
    delete(format!("/api/final-fantasy/character/{id}")).await
}

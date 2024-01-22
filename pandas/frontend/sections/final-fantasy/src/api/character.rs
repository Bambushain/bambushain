use bamboo_common::core::entities::*;
use bamboo_pandas_frontend_base::api::{delete, get, post, put_no_content, BambooApiResult};

pub async fn get_characters() -> BambooApiResult<Vec<Character>> {
    log::debug!("Get character");
    get("/api/final-fantasy/character").await
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

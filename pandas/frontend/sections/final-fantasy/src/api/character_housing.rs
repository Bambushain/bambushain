use bamboo_common::core::entities::*;
use bamboo_pandas_frontend_base::api::{delete, get, post, put_no_content, BambooApiResult};

pub async fn get_character_housing(character_id: i32) -> BambooApiResult<Vec<CharacterHousing>> {
    log::debug!("Get character housing");
    get(format!(
        "/api/final-fantasy/character/{character_id}/housing"
    ))
    .await
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

use bamboo_entities::prelude::*;
use bamboo_frontend_base_api::{delete, get, post, put_no_content, BambooApiResult};

pub async fn get_fighters(character_id: i32) -> BambooApiResult<Vec<Fighter>> {
    log::debug!("Get fighter");
    get(format!(
        "/api/final-fantasy/character/{character_id}/fighter"
    ))
    .await
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

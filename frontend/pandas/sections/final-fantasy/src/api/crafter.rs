use bamboo_common::core::entities::*;
use bamboo_common::frontend::api::BambooApiResult;
use bamboo_frontend_pandas_base::api::{delete, get, post, put_no_content};

pub async fn get_crafters(character_id: i32) -> BambooApiResult<Vec<Crafter>> {
    log::debug!("Get crafter");
    get(format!(
        "/api/final-fantasy/character/{character_id}/crafter"
    ))
    .await
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

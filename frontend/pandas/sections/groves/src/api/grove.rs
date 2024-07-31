use bamboo_common::core::entities::grove::{CreateGrove, JoinGrove};
use bamboo_common::core::entities::user::JoinStatus;
use bamboo_common::core::entities::Grove;
use bamboo_common::frontend::api;
use bamboo_common::frontend::api::BambooApiResult;

pub async fn get_groves() -> BambooApiResult<Vec<Grove>> {
    log::debug!("Get all groves for the current user");
    api::get("/api/grove").await
}

pub async fn get_grove(id: i32) -> BambooApiResult<Grove> {
    log::debug!("Get grove with id {id}");
    api::get(format!("/api/grove/{id}")).await
}

pub async fn save_grove_mods(id: i32, mods: &Vec<i32>) -> BambooApiResult<()> {
    log::debug!("Update grove mods for id {id}");
    api::put_no_content(format!("/api/grove/{id}/mods"), mods).await
}

pub async fn delete_grove(id: i32) -> BambooApiResult<()> {
    log::debug!("Delete grove with id {id}");
    api::delete(format!("/api/grove/{id}")).await
}

pub async fn enable_invite(id: i32) -> BambooApiResult<()> {
    log::debug!("Enable invite for id {id}");
    api::put_no_body_no_content(format!("/api/grove/{id}/invite")).await
}

pub async fn disable_invite(id: i32) -> BambooApiResult<()> {
    log::debug!("Disable invite for id {id}");
    api::delete(format!("/api/grove/{id}/invite")).await
}

pub async fn create_grove(name: String, invite_on: bool) -> BambooApiResult<Grove> {
    log::debug!("Create grove {name} with invites on {invite_on}");
    api::post("/api/grove", &CreateGrove { name, invite_on }).await
}

pub async fn join_grove(id: i32, invite_secret: String) -> BambooApiResult<()> {
    log::debug!("Join grove {id} with secret {invite_secret}");
    api::post_no_content(
        format!("/api/grove/{id}/join"),
        &JoinGrove { invite_secret },
    )
    .await
}

pub async fn check_join_status(id: i32) -> BambooApiResult<JoinStatus> {
    log::debug!("Check join status for grove {id}");
    api::get(format!("/api/grove/{id}/join")).await
}

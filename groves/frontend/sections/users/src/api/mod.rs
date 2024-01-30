use bamboo_common::core::entities::{Grove, GroveUser};
use bamboo_common::frontend::api::{delete, get, BambooApiResult};

pub async fn get_users(id: i32) -> BambooApiResult<Vec<GroveUser>> {
    log::debug!("Get all users for grove {id}");
    get(format!("/api/grove/{id}/user")).await
}

pub async fn get_grove(id: i32) -> BambooApiResult<Grove> {
    log::debug!("Get grove {id}");
    get(format!("/api/grove/{id}")).await
}

pub async fn delete_user(grove_id: i32, id: i32) -> BambooApiResult<()> {
    log::debug!("Delete user {id}");
    delete(format!("/api/grove/{grove_id}/user/{id}")).await
}

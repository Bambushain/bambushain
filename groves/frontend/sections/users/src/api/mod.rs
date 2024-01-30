use bamboo_common::core::entities::{Grove, GroveUser};
use bamboo_common::frontend::api::{get, put_no_body_no_content, BambooApiResult};

pub async fn get_users(id: i32) -> BambooApiResult<Vec<GroveUser>> {
    log::debug!("Get all users for grove {id}");
    get(format!("/api/grove/{id}/user")).await
}

pub async fn get_grove(id: i32) -> BambooApiResult<Grove> {
    log::debug!("Get grove {id}");
    get(format!("/api/grove/{id}")).await
}

pub async fn reset_password(grove_id: i32, id: i32) -> BambooApiResult<()> {
    log::debug!("Reset password for user {id}");
    put_no_body_no_content(format!("/api/grove/{grove_id}/user/{id}/password")).await
}

pub async fn make_user_mod(grove_id: i32, id: i32) -> BambooApiResult<()> {
    log::debug!("Make user {id} mod");
    put_no_body_no_content(format!("/api/grove/{grove_id}/user/{id}/mod")).await
}

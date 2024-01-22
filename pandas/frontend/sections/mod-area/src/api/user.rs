use bamboo_common::core::entities::*;
use bamboo_common::frontend::api::BambooApiResult;

use crate::api::{delete, get, post, put_no_body_no_content, put_no_content};

pub async fn get_users() -> BambooApiResult<Vec<WebUser>> {
    log::debug!("Get users");
    get("/api/user").await
}

pub async fn create_user(user: User) -> BambooApiResult<WebUser> {
    log::debug!("Create user {}", user.email);
    post("/api/user", &user).await
}

pub async fn make_user_mod(id: i32) -> BambooApiResult<()> {
    log::debug!("Make user {id} mod");
    put_no_body_no_content(format!("/api/user/{id}/mod")).await
}

pub async fn remove_user_mod(id: i32) -> BambooApiResult<()> {
    log::debug!("Remove user {id} mod");
    delete(format!("/api/user/{id}/mod")).await
}

pub async fn delete_user(id: i32) -> BambooApiResult<()> {
    log::debug!("Delete user {id}");
    delete(format!("/api/user/{id}")).await
}

pub async fn change_user_password(id: i32) -> BambooApiResult<()> {
    log::debug!("Change user {id} password");
    put_no_body_no_content(format!("/api/user/{id}/password")).await
}

pub async fn update_profile(id: i32, profile: UpdateProfile) -> BambooApiResult<()> {
    log::debug!(
        "Update profile of user {id} to the following data {:?}",
        profile
    );
    put_no_content(format!("/api/user/{id}/profile"), &profile).await
}

pub async fn disable_totp(id: i32) -> BambooApiResult<()> {
    log::debug!("Disable totp for user {id}");
    delete(format!("/api/user/{id}/totp")).await
}

use bamboo_common::core::entities::*;
use bamboo_common::frontend::api::{
    delete, get_with_query, put_no_body_no_content, BambooApiResult,
};

pub enum BannedStatus {
    Banned,
    Unbanned,
    All,
}

pub async fn get_users(
    grove_id: i32,
    banned_status: BannedStatus,
) -> BambooApiResult<Vec<user::GroveUser>> {
    log::debug!("Get users");
    let query = match banned_status {
        BannedStatus::Banned => {
            vec![
                ("grove", grove_id.to_string()),
                ("banned", true.to_string()),
            ]
        }
        BannedStatus::Unbanned => {
            vec![
                ("grove", grove_id.to_string()),
                ("banned", false.to_string()),
            ]
        }
        BannedStatus::All => {
            vec![("grove", grove_id.to_string()), ("all", true.to_string())]
        }
    };
    get_with_query("/api/user", query).await
}

pub async fn ban_user(grove_id: i32, user_id: i32) -> BambooApiResult<()> {
    log::debug!("Ban user {user_id} in {grove_id}");
    put_no_body_no_content(format!("/api/grove/{grove_id}/user/{user_id}/ban")).await
}

pub async fn unban_user(grove_id: i32, user_id: i32) -> BambooApiResult<()> {
    log::debug!("Ban user {user_id} in {grove_id}");
    delete(format!("/api/grove/{grove_id}/user/{user_id}/ban")).await
}

use bamboo_common::core::entities::*;
use bamboo_common::frontend::api::{get_with_query, BambooApiResult};

pub async fn get_users(grove_id: i32) -> BambooApiResult<Vec<User>> {
    log::debug!("Get users");
    get_with_query("/api/user", vec![("grove", grove_id.to_string())]).await
}

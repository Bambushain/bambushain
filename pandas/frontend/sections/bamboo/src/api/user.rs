use bamboo_common::core::entities::*;
use bamboo_common::frontend::api::BambooApiResult;

use crate::api::get;

pub async fn get_users() -> BambooApiResult<Vec<User>> {
    log::debug!("Get users");
    get("/api/user").await
}

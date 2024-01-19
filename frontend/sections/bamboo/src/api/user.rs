use bamboo_entities::prelude::*;

use crate::api::{get, BambooApiResult};

pub async fn get_users() -> BambooApiResult<Vec<WebUser>> {
    log::debug!("Get users");
    get("/api/user").await
}

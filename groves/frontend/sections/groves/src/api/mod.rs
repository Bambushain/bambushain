use bamboo_common::core::entities::Grove;
use bamboo_common::frontend::api::{get, BambooApiResult};

pub async fn get_groves() -> BambooApiResult<Vec<Grove>> {
    log::debug!("Get all groves");
    get("/api/grove").await
}

use bamboo_common::core::entities::Grove;
use bamboo_common::frontend::api;
use bamboo_common::frontend::api::BambooApiResult;

pub async fn get_groves() -> BambooApiResult<Vec<Grove>> {
    log::info!("Get all groves for the current user");

    api::get("/api/grove").await
}

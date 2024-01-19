use bamboo_entities::prelude::Grove;
use bamboo_frontend_base_api as api;

pub async fn get_grove() -> api::BambooApiResult<Grove> {
    log::debug!("Loading grove of current user");
    api::get("/api/grove").await
}

use bamboo_common::core::entities::Grove;
use bamboo_common::frontend::api::BambooApiResult;
use bamboo_pandas_frontend_base::api;

pub async fn get_grove() -> BambooApiResult<Grove> {
    log::debug!("Loading grove of current user");
    api::get("/api/grove").await
}

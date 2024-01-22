use bamboo_common::core::entities::Grove;
use bamboo_pandas_frontend_base::api;

pub async fn get_grove() -> api::BambooApiResult<Grove> {
    log::debug!("Loading grove of current user");
    api::get("/api/grove").await
}

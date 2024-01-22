use bamboo_common::core::entities::Grove;
use bamboo_common::frontend::api::BambooApiResult;
use bamboo_pandas_frontend_base::api;

pub async fn get_grove() -> BambooApiResult<Grove> {
    log::debug!("Loading grove of current user");
    api::get("/api/grove").await
}

pub async fn delete_grove() -> BambooApiResult<()> {
    log::debug!("Delete the current grove");
    api::delete("/api/grove").await
}

pub async fn disable_grove() -> BambooApiResult<()> {
    log::debug!("Disable the current grove");
    api::delete("/api/grove/enabled").await
}

pub async fn enable_grove() -> BambooApiResult<()> {
    log::debug!("Disable the current grove");
    api::put_no_body_no_content("/api/grove/enabled").await
}

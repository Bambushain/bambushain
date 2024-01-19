use bamboo_entities::prelude::Grove;
use bamboo_frontend_base_api as api;

pub async fn get_grove() -> api::BambooApiResult<Grove> {
    log::debug!("Loading grove of current user");
    api::get("/api/grove").await
}

pub async fn delete_grove() -> api::BambooApiResult<()> {
    log::debug!("Delete the current grove");
    api::delete("/api/grove").await
}

pub async fn disable_grove() -> api::BambooApiResult<()> {
    log::debug!("Disable the current grove");
    api::delete("/api/grove/enabled").await
}

pub async fn enable_grove() -> api::BambooApiResult<()> {
    log::debug!("Disable the current grove");
    api::put_no_body_no_content("/api/grove/enabled").await
}

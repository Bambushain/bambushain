use bamboo_common::core::entities::grove::CreateGroveRequest;
use bamboo_common::core::entities::Grove;
use bamboo_common::frontend::api::{get, post, BambooApiResult};

pub async fn get_groves() -> BambooApiResult<Vec<Grove>> {
    log::debug!("Get all groves");
    get("/api/grove").await
}

pub async fn create_grove(
    grove_name: String,
    mod_name: String,
    mod_email: String,
) -> BambooApiResult<Grove> {
    log::debug!("Create new grove {grove_name}");
    post(
        "/api/grove",
        &CreateGroveRequest::new(grove_name, mod_name, mod_email),
    )
    .await
}

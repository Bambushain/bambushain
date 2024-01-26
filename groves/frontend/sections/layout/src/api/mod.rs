use bamboo_common::frontend::api::{head, BambooApiResult};

pub async fn check_authentication() -> BambooApiResult<()> {
    log::debug!("Check authentication cookie");
    head("/api/login").await
}

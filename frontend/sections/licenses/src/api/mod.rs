use bamboo_entities::prelude::DependencyDetails;
use bamboo_frontend_base_api as api;

pub async fn get_licenses() -> api::BambooApiResult<Vec<DependencyDetails>> {
    log::debug!("Get licenses");
    api::get::<Vec<DependencyDetails>>("/api/licenses").await
}

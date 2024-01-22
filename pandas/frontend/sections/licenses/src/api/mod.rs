use bamboo_common::core::entities::DependencyDetails;
use bamboo_common::frontend::api::BambooApiResult;
use bamboo_pandas_frontend_base::api;

pub async fn get_licenses() -> BambooApiResult<Vec<DependencyDetails>> {
    log::debug!("Get licenses");
    api::get::<Vec<DependencyDetails>>("/api/licenses").await
}

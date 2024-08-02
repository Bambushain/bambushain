use bamboo_common::core::entities::*;
use bamboo_common::frontend::api::BambooApiResult;
use bamboo_frontend_pandas_base::api::*;

pub async fn send_support_request(request: SupportRequest) -> BambooApiResult<()> {
    log::debug!("Send support request");
    post_no_content("/api/support", &request).await
}

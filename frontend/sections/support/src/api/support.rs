use bamboo_entities::prelude::*;
use bamboo_frontend_base_api::*;

pub async fn send_support_request(request: SupportRequest) -> BambooApiResult<()> {
    log::debug!("Send support request");
    post_no_content("/api/support", &request).await
}

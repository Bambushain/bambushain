use bamboo_entities::prelude::*;

use crate::api;

pub async fn send_support_request(request: SupportRequest) -> api::BambooApiResult<()> {
    log::debug!("Send support request");
    api::post_no_content("/api/support", &request).await
}

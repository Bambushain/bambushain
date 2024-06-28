use actix_web::{post, web, HttpResponse};

use bamboo_common::backend::mailing;
use bamboo_common::backend::response::*;
use bamboo_common::backend::services::EnvService;
use bamboo_common::core::entities::{GlitchTipErrorRequest, SupportRequest};
use bamboo_common::core::error::*;

use crate::middleware::authenticate_user::{authenticate, Authentication};

#[post("/api/support", wrap = "authenticate!()")]
pub async fn send_support_request(
    authentication: Authentication,
    env_service: EnvService,
    body: Option<web::Json<SupportRequest>>,
) -> BambooApiResponseResult {
    let body = check_missing_fields!(body, "support")?;

    mailing::support::send_support_request(
        authentication.user.clone(),
        env_service,
        body.into_inner(),
    )
    .await
    .map(|_| no_content!())
}

#[post("/api/glitchtip", wrap = "authenticate!()")]
pub async fn report_glitchtip_error(
    _body: Option<web::Json<GlitchTipErrorRequest>>,
) -> HttpResponse {
    no_content!()
}

use std::collections::BTreeMap;

use actix_web::{post, web, HttpResponse};
use sentry::protocol::{Event, Level};
use sentry::types::random_uuid;
use sentry::types::protocol::v7::Context;

use bamboo_entities::prelude::{GlitchTipErrorRequest, SupportRequest};
use bamboo_error::*;
use bamboo_services::prelude::EnvService;

use crate::mailing;
use crate::middleware::authenticate_user::{authenticate, Authentication};
use crate::response::macros::*;

#[post("/api/support", wrap = "authenticate!()")]
pub async fn send_support_request(
    authentication: Authentication,
    env_service: EnvService,
    body: Option<web::Json<SupportRequest>>,
) -> BambooApiResponseResult {
    let body = check_missing_fields!(body, "support")?;

    let html_body = format!(
        r#"
<html lang="de" style="font-family: system-ui,-apple-system,'Segoe UI','Roboto','Ubuntu','Cantarell','Noto Sans',sans-serif,'Apple Color Emoji','Segoe UI Emoji','Segoe UI Symbol','Noto Color Emoji';">
<head>
</head>
<body>
    {}
</body>
</html>"#,
        body.message
            .clone()
            .replace("\r\n", "<br>")
            .replace('\n', "<br>")
    );

    mailing::send_mail_with_reply_to(
        env_service,
        body.subject.clone(),
        "panda.helferlein@bambushain.app",
        authentication.user.email.clone(),
        body.message.clone(),
        html_body,
    )
    .await
    .map(|_| no_content!())
}

#[post("/api/glitchtip", wrap = "authenticate!()")]
pub async fn report_glitchtip_error(
    body: Option<web::Json<GlitchTipErrorRequest>>,
) -> HttpResponse {
    if let Some(body) = body {
        let event_id = random_uuid();
        let mut base_data = BTreeMap::new();
        base_data.insert("form".to_string(), serde_json::Value::String(body.form.clone()));
        base_data.insert("page".to_string(), serde_json::Value::String(body.page.clone()));
        base_data.insert("full_url".to_string(), serde_json::Value::String(body.full_url.clone()));

        let mut error = BTreeMap::new();
        error.insert("entity_type".to_string(), serde_json::Value::String(body.bamboo_error.entity_type.to_string()));
        error.insert("error_type".to_string(), serde_json::Value::String(body.bamboo_error.error_type.to_string()));
        error.insert("message".to_string(), serde_json::Value::String(body.bamboo_error.message.to_string()));

        let mut contexts = BTreeMap::new();
        contexts.insert("base_data".to_string(), Context::Other(base_data));
        contexts.insert("bamboo_error".to_string(), Context::Other(error));

        let event = Event {
            event_id,
            message: Some("Unknown error occured".into()),
            level: Level::Error,
            logger: Some("bamboo-web".into()),
            contexts,
            ..Default::default()
        };
        sentry::capture_event(event.clone());
    }

    no_content!()
}

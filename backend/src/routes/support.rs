use actix_web::{post, web};

use bamboo_entities::prelude::SupportRequest;
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

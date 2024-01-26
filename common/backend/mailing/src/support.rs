use crate::mailer::send_mail_with_reply_to;
use bamboo_common_backend_services::EnvService;
use bamboo_common_core::entities::{SupportRequest, User};
use bamboo_common_core::error::BambooErrorResult;

pub async fn send_support_request(
    user: User,
    env_service: EnvService,
    support_request: SupportRequest,
) -> BambooErrorResult {
    let html_body = format!(
        r#"
<html lang="de" style="font-family: system-ui,-apple-system,'Segoe UI','Roboto','Ubuntu','Cantarell','Noto Sans',sans-serif,'Apple Color Emoji','Segoe UI Emoji','Segoe UI Symbol','Noto Color Emoji';">
<head>
</head>
<body>
    {}
</body>
</html>"#,
        support_request
            .message
            .clone()
            .replace("\r\n", "<br>")
            .replace('\n', "<br>")
    );

    send_mail_with_reply_to(
        env_service,
        support_request.subject.clone(),
        "panda.helferlein@bambushain.app",
        user.email.clone(),
        support_request.message.clone(),
        html_body,
    )
    .await
    .map(|_| ())
}

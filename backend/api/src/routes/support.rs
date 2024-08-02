use actix_web::{post, web};
use bamboo_common::backend::mailing;
use bamboo_common::backend::response::*;
use bamboo_common::core::entities::SupportRequest;
use bamboo_common::core::error::*;
use maud::{html, PreEscaped};

use bamboo_common::backend::actix::middleware::{authenticate, Authentication};
use bamboo_common::backend::mailing::Mail;

#[post("/api/support", wrap = "authenticate!()")]
pub async fn send_support_request(
    authentication: Authentication,
    body: Option<web::Json<SupportRequest>>,
) -> BambooApiResponseResult {
    let body = check_missing_fields!(body, "support")?.into_inner();

    let mail_body = html! {
        html lang="de" style="font-family: system-ui,-apple-system,'Segoe UI','Roboto','Ubuntu','Cantarell','Noto Sans',sans-serif,'Apple Color Emoji','Segoe UI Emoji','Segoe UI Symbol','Noto Color Emoji';" {
            head {}
            body {
                (PreEscaped(body.message.replace("\r\n", "<br>").replace('\n', "<br>")))
            }
        }
    }.into_string();

    mailing::enqueue_mail(Mail::new(
        body.subject,
        "panda.helferlein@bambushain.app",
        mail_body,
        Some(authentication.user.email.clone()),
    ))
    .await;

    Ok(no_content!())
}

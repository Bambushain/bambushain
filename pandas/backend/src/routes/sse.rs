use actix_web::{get, Responder};

use crate::middleware::authenticate_user::{authenticate, Authentication};
use crate::notifier::Notifier;

#[get("/sse/event", wrap = "authenticate!()")]
pub async fn event_sse_client(
    notifier: Notifier,
    authentication: Authentication,
) -> impl Responder {
    log::debug!("Register new event sse client");
    notifier.new_client(authentication.user.clone()).await
}

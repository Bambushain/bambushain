use actix_web::{get, Responder};

use crate::middleware::authenticate_user::authenticate;
use crate::sse::Notification;

#[get("/sse/event", wrap = "authenticate!()")]
pub async fn event_sse_client(notification_state: Notification) -> impl Responder {
    log::debug!("Register new event sse client");
    notification_state.event_broadcaster.new_client().await
}

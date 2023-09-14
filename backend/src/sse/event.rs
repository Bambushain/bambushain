use actix_web::Responder;

use crate::sse::NotificationState;

pub async fn event_sse_client(
    notification_state: actix_web::web::Data<NotificationState>,
) -> impl Responder {
    log::debug!("Register new event sse client");
    notification_state.event_broadcaster.new_client().await
}

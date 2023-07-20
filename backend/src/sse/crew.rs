use actix_web::Responder;

use crate::sse::NotificationState;

pub async fn crew_sse_client(notification_state: actix_web::web::Data<NotificationState>) -> impl Responder {
    log::debug!("Register new crew sse client");
    notification_state.crew_broadcaster.new_client().await
}
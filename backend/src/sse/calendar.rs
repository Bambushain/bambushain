use actix_web::Responder;

use crate::sse::NotificationState;

pub async fn calendar_sse_client(notification_state: actix_web::web::Data<NotificationState>) -> impl Responder {
    log::debug!("Register new calendar sse client");
    notification_state.calendar_broadcaster.new_client().await
}

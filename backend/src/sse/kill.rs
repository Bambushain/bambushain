use actix_web::Responder;

use crate::sse::NotificationState;

pub async fn kill_sse_client(notification_state: actix_web::web::Data<NotificationState>) -> impl Responder {
    log::debug!("Register new kill sse client");
    notification_state.kill_broadcaster.new_client().await
}

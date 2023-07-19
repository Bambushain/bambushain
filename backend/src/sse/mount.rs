use actix_web::Responder;

use crate::sse::NotificationState;

pub async fn mount_sse_client(notification_state: actix_web::web::Data<NotificationState>) -> impl Responder {
    log::debug!("Register new mount sse client");
    notification_state.mount_broadcaster.new_client().await
}

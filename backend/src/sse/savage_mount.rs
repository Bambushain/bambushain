use actix_web::Responder;

use crate::sse::NotificationState;

pub async fn savage_mount_sse_client(notification_state: actix_web::web::Data<NotificationState>) -> impl Responder {
    log::debug!("Register new savage mount sse client");
    notification_state.savage_mount_broadcaster.new_client().await
}

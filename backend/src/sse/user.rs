use actix_web::Responder;

use crate::sse::NotificationState;

pub async fn user_sse_client(
    notification_state: actix_web::web::Data<NotificationState>,
) -> impl Responder {
    log::debug!("Register new user sse client");
    notification_state.user_broadcaster.new_client().await
}

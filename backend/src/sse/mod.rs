use std::sync::Arc;
use actix_web::web;

use crate::broadcaster::user::UserBroadcaster;

pub mod crew;

pub struct NotificationState {
    pub crew_broadcaster: Arc<UserBroadcaster>,
}

pub type Notification = web::Data<NotificationState>;
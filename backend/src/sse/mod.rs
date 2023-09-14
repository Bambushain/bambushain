use std::sync::Arc;

use actix_web::web;

use crate::broadcaster::event::EventBroadcaster;
use crate::broadcaster::user::UserBroadcaster;

pub mod event;
pub mod user;

pub struct NotificationState {
    pub user_broadcaster: Arc<UserBroadcaster>,
    pub event_broadcaster: Arc<EventBroadcaster>,
}

pub type Notification = web::Data<NotificationState>;

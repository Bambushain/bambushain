use std::sync::Arc;

use actix_web::web;

use crate::broadcaster::event::EventBroadcaster;

pub mod event;

pub struct NotificationState {
    pub event_broadcaster: Arc<EventBroadcaster>,
}

pub type Notification = web::Data<NotificationState>;

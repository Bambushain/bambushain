use std::sync::Arc;

use actix_web::web;

use crate::broadcaster::event::EventBroadcaster;

pub mod event;

pub struct NotificationState {
    pub event_broadcaster: Arc<EventBroadcaster>,
}

impl NotificationState {
    pub fn new() -> Self {
        let event_broadcaster = EventBroadcaster::create();

        Self {
            event_broadcaster
        }
    }
}

pub type Notification = web::Data<NotificationState>;

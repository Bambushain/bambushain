use std::sync::Arc;

use actix_web::{web, Responder};

use crate::notifier::event::EventBroadcaster;
use bamboo_common::core::entities::{Event, User};

#[derive(Clone)]
pub struct NotifierState {
    event_broadcaster: Arc<EventBroadcaster>,
}

impl NotifierState {
    pub fn new() -> Self {
        let event_broadcaster = EventBroadcaster::create();

        Self { event_broadcaster }
    }

    pub fn notify_event_create(&self, event: Event) {
        log::info!("Event created, notify sources");
        self.event_broadcaster.notify_create(event)
    }

    pub fn notify_event_update(&self, event: Event) {
        log::info!("Event updated, notify sources");
        self.event_broadcaster.notify_update(event)
    }

    pub fn notify_event_delete(&self, event: Event) {
        log::info!("Event deleted, notify sources");
        self.event_broadcaster.notify_delete(event)
    }

    pub async fn new_client(&self, user: User) -> impl Responder {
        log::info!("Wanted new client");
        self.event_broadcaster.new_client(user).await
    }
}

impl Default for NotifierState {
    fn default() -> Self {
        Self::new()
    }
}

pub type Notifier = web::Data<NotifierState>;

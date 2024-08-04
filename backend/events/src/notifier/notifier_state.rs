use std::sync::Arc;

use crate::notifier::event::EventBroadcaster;
use actix_web::{web, Responder};
use bamboo_common::backend::notification::EventAction;
use bamboo_common::core::entities::grove::Model;
use bamboo_common::core::entities::User;

#[derive(Clone)]
pub struct NotifierState {
    event_broadcaster: Arc<EventBroadcaster>,
}

impl NotifierState {
    pub(crate) async fn send_event(&self, event_action: EventAction, groves: Vec<Model>) {
        self.event_broadcaster
            .send_event(event_action, groves)
            .await
    }
}

impl NotifierState {
    pub fn new() -> Self {
        let event_broadcaster = EventBroadcaster::create();

        Self { event_broadcaster }
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

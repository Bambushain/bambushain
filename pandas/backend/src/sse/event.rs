use std::fmt::Display;

use actix_web_lab::sse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum EventAction {
    Created,
    Updated,
    Deleted,
}

impl Display for EventAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match self {
            Self::Created => "created",
            Self::Updated => "updated",
            Self::Deleted => "deleted",
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Event {
    pub event: bamboo_common::core::entities::Event,
    pub action: EventAction,
}

impl From<Event> for sse::Event {
    fn from(value: Event) -> Self {
        let mut data = sse::Data::new_json(value.event.clone()).unwrap();
        data.set_event(value.action.to_string());

        sse::Event::Data(data)
    }
}

impl Event {
    fn new(action: EventAction, event: bamboo_common::core::entities::Event) -> Self {
        Self { event, action }
    }

    pub fn created(event: bamboo_common::core::entities::Event) -> Self {
        Self::new(EventAction::Created, event)
    }

    pub fn updated(event: bamboo_common::core::entities::Event) -> Self {
        Self::new(EventAction::Updated, event)
    }

    pub fn deleted(event: bamboo_common::core::entities::Event) -> Self {
        Self::new(EventAction::Deleted, event)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Comment {
    Connected,
    Ping,
}

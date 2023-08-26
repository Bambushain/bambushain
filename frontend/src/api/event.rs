use std::rc::Rc;

use async_trait::async_trait;
use bounce::BounceStates;
use bounce::query::{Query, QueryResult};
use date_range::DateRange;

use pandaparty_entities::prelude::*;

use crate::api::{ApiError, delete, get_with_query, post, put, PandapartyApiResult};

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct EventRange {
    pub events: Vec<Event>,
}

impl From<Vec<Event>> for EventRange {
    fn from(value: Vec<Event>) -> Self {
        Self {
            events: value,
        }
    }
}

async fn get_events(range: Rc<DateRange>) -> PandapartyApiResult<Vec<Event>> {
    log::debug!("Get events");
    get_with_query("/api/event", vec![
        ("start", range.since().format("%F").to_string().as_str()),
        ("end", range.until().format("%F").to_string().as_str()),
    ]).await
}

#[async_trait(? Send)]
impl Query for EventRange {
    type Input = DateRange;
    type Error = ApiError;

    async fn query(_states: &BounceStates, input: Rc<Self::Input>) -> QueryResult<Self> {
        get_events(input).await.map(|event| Rc::new(event.into()))
    }
}

pub async fn create_event(event: Event) -> PandapartyApiResult<Event> {
    log::debug!("Create event {}", event.title);
    post("/api/event", &event).await
}

pub async fn update_event(id: i32, event: Event) -> PandapartyApiResult<()> {
    log::debug!("Update event {id}");
    put(format!("/api/event/{id}"), &event).await
}

pub async fn delete_event(id: i32) -> PandapartyApiResult<()> {
    log::debug!("Delete event {id}");
    delete(format!("/api/event/{id}")).await
}
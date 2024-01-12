use std::rc::Rc;

use date_range::DateRange;

use bamboo_entities::prelude::*;
use bamboo_frontend_base_api::{BambooApiResult, delete, get_with_query, post, put_no_content};

pub async fn get_events(range: Rc<DateRange>) -> BambooApiResult<Vec<Event>> {
    log::debug!("Get events");
    get_with_query(
        "/api/bamboo-grove/event",
        vec![
            ("start", range.since().format("%F").to_string().as_str()),
            ("end", range.until().format("%F").to_string().as_str()),
        ],
    )
        .await
}

pub async fn create_event(event: Event) -> BambooApiResult<Event> {
    log::debug!("Create event {}", event.title);
    post("/api/bamboo-grove/event", &event).await
}

pub async fn update_event(id: i32, event: Event) -> BambooApiResult<()> {
    log::debug!("Update event {id}");
    put_no_content(format!("/api/bamboo-grove/event/{id}"), &event).await
}

pub async fn delete_event(id: i32) -> BambooApiResult<()> {
    log::debug!("Delete event {id}");
    delete(format!("/api/bamboo-grove/event/{id}")).await
}

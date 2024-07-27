use std::rc::Rc;

use date_range::DateRange;

use bamboo_common::core::entities::event::GroveEvent;
use bamboo_common::frontend::api::{delete, get_with_query, post, put_no_content, BambooApiResult};

pub async fn get_events(
    range: Rc<DateRange>,
    grove_id: Option<i32>,
) -> BambooApiResult<Vec<GroveEvent>> {
    log::debug!("Get events");
    if let Some(grove_id) = grove_id {
        get_with_query(
            "/api/bamboo-grove/event",
            vec![
                ("start", range.since().format("%F").to_string().as_str()),
                ("end", range.until().format("%F").to_string().as_str()),
                ("grove", grove_id.to_string().as_str()),
            ],
        )
        .await
    } else {
        get_with_query(
            "/api/bamboo-grove/event",
            vec![
                ("start", range.since().format("%F").to_string().as_str()),
                ("end", range.until().format("%F").to_string().as_str()),
            ],
        )
        .await
    }
}

pub async fn create_event(event: GroveEvent) -> BambooApiResult<GroveEvent> {
    log::debug!("Create event {}", event.title);
    post("/api/bamboo-grove/event", &event).await
}

pub async fn update_event(id: i32, event: GroveEvent) -> BambooApiResult<()> {
    log::debug!("Update event {id}");
    put_no_content(format!("/api/bamboo-grove/event/{id}"), &event).await
}

pub async fn delete_event(id: i32) -> BambooApiResult<()> {
    log::debug!("Delete event {id}");
    delete(format!("/api/bamboo-grove/event/{id}")).await
}

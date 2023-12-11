use actix_web::{delete, get, post, put, web};
use chrono::NaiveDate;
use date_range::DateRange;
use serde::Deserialize;

use bamboo_dbal::prelude::dbal;
use bamboo_entities::prelude::Event;
use bamboo_error::*;
use bamboo_services::prelude::DbConnection;

use crate::middleware::authenticate_user::authenticate;
use crate::notifier;
use crate::path;
use crate::response::macros::*;

#[derive(Deserialize)]
pub struct GetEventsQuery {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

#[get("/api/bamboo-grove/event", wrap = "authenticate!()")]
pub async fn get_events(
    query: Option<web::Query<GetEventsQuery>>,
    db: DbConnection,
) -> BambooApiResponseResult {
    let query = check_invalid_query!(query, "event")?;

    let range = DateRange::new(query.start, query.end).map_err(|_| {
        BambooError::invalid_data("event", "The start date cannot be after the end date")
    })?;

    dbal::get_events(range, &db).await.map(|data| list!(data))
}

#[post("/api/bamboo-grove/event", wrap = "authenticate!()")]
pub async fn create_event(
    body: Option<web::Json<Event>>,
    notifier: notifier::Notifier,
    db: DbConnection,
) -> BambooApiResult<Event> {
    let body = check_missing_fields!(body, "event")?;

    let data = dbal::create_event(body.into_inner(), &db).await?;
    notifier.notify_event_create(data.clone());

    Ok(created!(data))
}

#[put("/api/bamboo-grove/event/{event_id}", wrap = "authenticate!()")]
pub async fn update_event(
    path: Option<path::EventPath>,
    body: Option<web::Json<Event>>,
    notifier: notifier::Notifier,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "event")?;
    let body = check_missing_fields!(body, "event")?;

    dbal::update_event(path.event_id, body.into_inner(), &db).await?;

    let event = dbal::get_event(path.event_id, &db).await?;
    notifier.notify_event_update(event);

    Ok(no_content!())
}

#[delete("/api/bamboo-grove/event/{event_id}", wrap = "authenticate!()")]
pub async fn delete_event(
    path: Option<path::EventPath>,
    notifier: notifier::Notifier,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "event")?;

    let event = dbal::get_event(path.event_id, &db).await?;
    dbal::delete_event(path.event_id, &db).await?;
    notifier.notify_event_delete(event);

    Ok(no_content!())
}

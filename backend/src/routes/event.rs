use actix_web::{delete, get, post, put, web, HttpResponse};
use chrono::NaiveDate;
use date_range::DateRange;
use serde::Deserialize;

use bamboo_entities::prelude::Event;
use bamboo_error::*;
use bamboo_services::prelude::DbConnection;

use crate::middleware::authenticate_user::authenticate;
use crate::sse::Notification;

#[derive(Deserialize)]
pub struct GetEventsQuery {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

#[derive(Deserialize)]
pub struct EventPath {
    pub id: i32,
}

#[get("/api/bamboo-grove/event", wrap = "authenticate!()")]
pub async fn get_events(
    query: Option<web::Query<GetEventsQuery>>,
    db: DbConnection,
) -> HttpResponse {
    let query = check_invalid_query!(query, "event");

    let range = match DateRange::new(query.start, query.end) {
        Ok(range) => range,
        Err(_) => {
            return bad_request!(bamboo_invalid_data_error!(
                "event",
                "The start date cannot be after the end date"
            ))
        }
    };

    ok_or_error!(bamboo_dbal::event::get_events(range, &db).await)
}

#[post("/api/bamboo-grove/event", wrap = "authenticate!()")]
pub async fn create_event(
    body: Option<web::Json<Event>>,
    notification: Notification,
    db: DbConnection,
) -> HttpResponse {
    let body = check_missing_fields!(body, "event");

    let data = bamboo_dbal::event::create_event(body.into_inner(), &db).await;
    if data.is_ok() {
        actix_web::rt::spawn(async move {
            notification.event_broadcaster.notify_change().await;
        });
    }

    created_or_error!(data)
}

#[put("/api/bamboo-grove/event/{id}", wrap = "authenticate!()")]
pub async fn update_event(
    path: Option<web::Path<EventPath>>,
    body: Option<web::Json<Event>>,
    notification: Notification,
    db: DbConnection,
) -> HttpResponse {
    let path = check_invalid_path!(path, "event");
    let body = check_missing_fields!(body, "event");

    let data = bamboo_dbal::event::update_event(path.id, body.into_inner(), &db).await;
    if data.is_ok() {
        actix_web::rt::spawn(async move {
            notification.event_broadcaster.notify_change().await;
        });
    }

    no_content_or_error!(data)
}

#[delete("/api/bamboo-grove/event/{id}", wrap = "authenticate!()")]
pub async fn delete_event(path: Option<web::Path<EventPath>>, db: DbConnection) -> HttpResponse {
    let path = check_invalid_path!(path, "event");

    no_content_or_error!(bamboo_dbal::event::delete_event(path.id, &db).await)
}

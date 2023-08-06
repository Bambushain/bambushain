use actix_web::{HttpResponse, web};
use chrono::NaiveDate;
use date_range::DateRange;
use serde::Deserialize;

use pandaparty_entities::pandaparty_invalid_data_error;
use pandaparty_entities::prelude::Event;

use crate::DbConnection;
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

pub async fn get_events(query: web::Query<GetEventsQuery>, db: DbConnection) -> HttpResponse {
    let range = match DateRange::new(query.start, query.end) {
        Ok(range) => range,
        Err(_) => return bad_request!(pandaparty_invalid_data_error!("event", "The start date cannot be after the end date"))
    };

    ok_or_error!(pandaparty_dbal::event::get_events(range, &db).await)
}

pub async fn create_event(body: web::Json<Event>, notification: Notification, db: DbConnection) -> HttpResponse {
    let data = pandaparty_dbal::event::create_event(body.into_inner(), &db).await;
    if data.is_ok() {
        actix_web::rt::spawn(async move {
            notification.event_broadcaster.notify_change().await;
        });
    }

    created_or_error!(data)
}

pub async fn update_event(path: web::Path<EventPath>, body: web::Json<Event>, notification: Notification, db: DbConnection) -> HttpResponse {
    let data = pandaparty_dbal::event::update_event(path.id, body.into_inner(), &db).await;
    if data.is_ok() {
        actix_web::rt::spawn(async move {
            notification.event_broadcaster.notify_change().await;
        });
    }
    
    no_content_or_error!(data)
}

pub async fn delete_event(path: web::Path<EventPath>, db: DbConnection) -> HttpResponse {
    no_content_or_error!(pandaparty_dbal::event::delete_event(path.id, &db).await)
}

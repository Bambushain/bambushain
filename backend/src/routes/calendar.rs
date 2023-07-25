use actix_web::{HttpRequest, HttpResponse};
use actix_web::web;
use chrono::{Datelike, NaiveDate, Utc};
use serde::Deserialize;

use sheef_dbal::prelude::*;
use sheef_entities::prelude::*;

use crate::sse::NotificationState;

pub fn get_current_month() -> u32 {
    Utc::now().date_naive().month()
}

pub fn get_current_year() -> i32 {
    Utc::now().date_naive().year()
}

#[derive(Deserialize)]
pub struct CalendarQueryInfo {
    #[serde(default = "get_current_year")]
    pub year: i32,
    #[serde(default = "get_current_month")]
    pub month: u32,
}

#[derive(Deserialize)]
pub struct DayDetailsPathInfo {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

macro_rules! date_from_values {
    ($year:expr, $month:expr, $day:expr) => {
        {
            if let Some(date) = NaiveDate::from_ymd_opt($year, $month, $day) {
                date
            } else {
                return no_content_or_error!(Err::<(), SheefError>(sheef_invalid_data_error!("calendar", "The date is invalid")));
            }
        }
    };
    ($year:expr, $month:expr) => {
        date_from_values!($year, $month, 1)
    };
}

pub async fn get_calendar(query: web::Query<CalendarQueryInfo>) -> HttpResponse {
    date_from_values!(query.year, query.month);

    ok_or_error!(get_events_for_month(query.year, query.month).await)
}

pub async fn get_day_details(path: web::Path<DayDetailsPathInfo>, req: HttpRequest) -> HttpResponse {
    let date = date_from_values!(path.year, path.month, path.day);
    let username = username!(req);

    ok_or_error!(get_event(username.clone(), date).await)
}

pub async fn update_day_details(path: web::Path<DayDetailsPathInfo>, body: web::Json<SetEvent>, notification_state: web::Data<NotificationState>, req: HttpRequest) -> HttpResponse {
    let date = date_from_values!(path.year, path.month, path.day);
    let username = username!(req);

    let data = set_event(username.to_string(), body.into_inner().clone(), date).await;
    actix_web::rt::spawn(async move {
        notification_state.calendar_broadcaster.notify_change().await;
    });

    no_content_or_error!(data)
}

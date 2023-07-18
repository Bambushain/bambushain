use actix_web::{HttpRequest, HttpResponse};
use actix_web::http::StatusCode;
use actix_web::web::{Json, Path, Query};
use chrono::{Datelike, NaiveDate, Utc};
use serde::Deserialize;
use sheef_entities::event::SetEvent;

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
                return HttpResponse::new(StatusCode::UNPROCESSABLE_ENTITY);
            }
        }
    };
    ($year:expr, $month:expr) => {
        date_from_values!($year, $month, 1)
    };
}

pub async fn get_calendar(query: Query<CalendarQueryInfo>) -> HttpResponse {
    date_from_values!(query.year, query.month);

    let data = sheef_database::event::get_events_for_month(query.year, query.month).await;
    if let Some(calendar) = data {
        ok_json!(calendar)
    } else {
        no_content!()
    }
}

pub async fn get_day_details(path: Path<DayDetailsPathInfo>, req: HttpRequest) -> HttpResponse {
    let date = date_from_values!(path.year, path.month, path.day);
    let username = username!(req);
    let data = sheef_database::event::get_event(&username, &date).await;
    if let Some(event) = data {
        ok_json!(event)
    } else {
        not_found!()
    }
}

pub async fn update_day_details(path: Path<DayDetailsPathInfo>, body: Json<SetEvent>, req: HttpRequest) -> HttpResponse {
    let date = date_from_values!(path.year, path.month, path.day);
    let username = username!(req);
    let data = sheef_database::event::set_event(&username, &body.time, body.available, &date).await;
    if data.is_some() {
        no_content!()
    } else {
        not_found!()
    }
}

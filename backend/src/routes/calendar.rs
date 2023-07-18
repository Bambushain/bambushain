use actix_web::{HttpRequest, HttpResponse};
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
                return no_content_or_error!(Err::<(), sheef_entities::SheefError>(sheef_entities::sheef_invalid_data_error!("calendar", "The date is invalid")));
            }
        }
    };
    ($year:expr, $month:expr) => {
        date_from_values!($year, $month, 1)
    };
}

pub async fn get_calendar(query: Query<CalendarQueryInfo>) -> HttpResponse {
    date_from_values!(query.year, query.month);

    ok_or_error!(sheef_database::event::get_events_for_month(query.year, query.month).await)
}

pub async fn get_day_details(path: Path<DayDetailsPathInfo>, req: HttpRequest) -> HttpResponse {
    let date = date_from_values!(path.year, path.month, path.day);
    let username = username!(req);

    ok_or_error!(sheef_database::event::get_event(&username, &date).await)
}

pub async fn update_day_details(path: Path<DayDetailsPathInfo>, body: Json<SetEvent>, req: HttpRequest) -> HttpResponse {
    let date = date_from_values!(path.year, path.month, path.day);
    let username = username!(req);

    no_content_or_error!(sheef_database::event::set_event(&username, &body.time, body.available, &date).await)
}

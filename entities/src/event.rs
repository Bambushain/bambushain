use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub username: String,
    pub time: String,
    pub available: bool,
    pub date: NaiveDate,
}

#[derive(Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Calendar {
    pub year: i32,
    pub month: u32,
    pub first_day: u32,
    pub last_day: u32,
    pub events: Vec<Event>,
}

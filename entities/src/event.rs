use std::cmp::Ordering;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub username: String,
    pub time: String,
    pub available: bool,
    pub date: NaiveDate,
}

impl PartialOrd<Self> for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.date.partial_cmp(&other.date).map(|o| o.then(self.username.to_lowercase().cmp(&other.username.to_lowercase())))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date.cmp(&other.date).then(self.username.to_lowercase().cmp(&other.username.to_lowercase()))
    }
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

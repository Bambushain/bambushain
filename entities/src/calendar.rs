use std::cmp::Ordering;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::event;

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Calendar {
    pub year: i32,
    pub month: u32,
    pub days: Vec<CalendarDay>,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Default)]
pub struct CalendarDay {
    pub date: NaiveDate,
    pub events: Vec<event::Model>,
}

impl PartialOrd for CalendarDay {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.date.partial_cmp(&other.date)
    }
}

impl Ord for CalendarDay {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date.cmp(&other.date)
    }
}

#[derive(Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetEvent {
    pub available: bool,
    #[serde(default)]
    pub time: String,
}
use std::ops::Deref;
use std::rc::Rc;

use async_trait::async_trait;
use bounce::BounceStates;
use bounce::query::{Query, QueryResult};
use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::api::{ApiError, get, put, SheefApiResult};

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct Calendar {
    pub calendar: sheef_entities::prelude::Calendar,
}

impl From<sheef_entities::prelude::Calendar> for Calendar {
    fn from(value: sheef_entities::prelude::Calendar) -> Self {
        Self {
            calendar: value
        }
    }
}

async fn get_calendar(year: i32, month: u32) -> SheefApiResult<sheef_entities::prelude::Calendar> {
    log::debug!("Loading calendar for {}-{}", year, month);
    get::<sheef_entities::prelude::Calendar>(format!("/api/calendar?year={}&month={}", year, month)).await
}

#[async_trait(? Send)]
impl Query for Calendar {
    type Input = (i32, u32);
    type Error = ApiError;

    async fn query(_states: &BounceStates, input: Rc<Self::Input>) -> QueryResult<Self> {
        let (year, month) = input.deref();
        get_calendar(*year, *month).await.map(|cal| Rc::new(cal.into()))
    }
}

pub async fn update_event_availability(set_event: sheef_entities::prelude::SetEvent, date: NaiveDate) -> SheefApiResult<()> {
    log::debug!("Update event availability on {} to {}", date, set_event.available);
    put(format!("/api/calendar/{}/{}/{}", date.year(), date.month(), date.day()), &set_event).await
}

use chrono::prelude::*;
use std::env::join_paths;
use std::fs::remove_file;
use chrono::{Days, Months};
use log::warn;
use serde::{Deserialize, Serialize};
use crate::database::{EmptyResult, persist_entity, read_entity, read_entity_dir, validate_database_dir};

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

fn validate_event_dir() -> String {
    let path = join_paths(vec![validate_database_dir(), "event".to_string()]).expect("Paths should be able to join").into_string().expect("String should be available as core string");
    let result = std::fs::create_dir_all(path.as_str());
    if result.is_err() {
        panic!("Failed to create event database dir {}", result.err().unwrap());
    }

    path
}

fn get_date_event_dir(date: &NaiveDate) -> Option<String> {
    let formatted_date = &date.format("%Y-%m-%d").to_string();
    let path = join_paths(vec![validate_event_dir(), formatted_date.to_string()]).expect("Paths should be able to join").into_string().expect("String should be available as core string");
    match std::fs::create_dir_all(path.as_str()) {
        Ok(_) => Some(path),
        Err(err) => {
            warn!("Failed to create event dir for date {}: {}", formatted_date, err);
            None
        }
    }
}

pub fn create_event(username: &String, time: &String, available: bool, date: &NaiveDate) -> Option<Event> {
    let event = Event {
        username: username.to_string(),
        time: time.to_string(),
        available: false,
        date: *date,
    };

    let event_dir = match get_date_event_dir(date) {
        Some(dir) => dir,
        None => {
            warn!("Failed to get user event dir ({})", username);
            return None;
        }
    };

    match persist_entity(event_dir, username, event) {
        Ok(event) => Some(event),
        Err(_) => None
    }
}

pub fn get_event(username: &String, date: &NaiveDate) -> Option<Event> {
    let event_dir = match get_date_event_dir(date) {
        Some(dir) => dir,
        None => {
            warn!("Failed to get user event dir");
            return None;
        }
    };

    read_entity(event_dir, username)
}

pub fn get_events_for_date(date: &NaiveDate) -> Option<impl Iterator<Item=Event>> {
    let event_dir = match get_date_event_dir(date) {
        Some(dir) => dir,
        None => {
            warn!("Failed to get date event dir ({})", date.format("%Y-%m-%d"));
            return None;
        }
    };

    read_entity_dir(event_dir)
}

pub fn get_events_for_month(year: i32, month: u32) -> Option<Calendar> {
    let first_day_of_month = match NaiveDate::from_ymd_opt(year, month, 1) {
        Some(day) => day,
        None => return None
    };
    let last_day_of_month = first_day_of_month.checked_add_months(Months::new(1)).expect("One month should be able to add").checked_sub_days(Days::new(1)).expect("One day should be able to subtract");

    let events = (first_day_of_month.day()..last_day_of_month.day()).map(|day| {
        let date = NaiveDate::from_ymd_opt(year, month, day).expect("Date should be valid");
        let event_dir = match get_date_event_dir(&date) {
            Some(dir) => dir,
            None => {
                warn!("Failed to get date event dir ({})", date.format("%Y-%m-%d"));
                return None;
            }
        };

        read_entity_dir::<Event>(event_dir)
    });

    let mut calendar = Calendar{
        year,
        month,
        first_day: 1,
        last_day: last_day_of_month.day(),
        events: vec![],
    };
    for date in events {
        if date.is_some() {
            calendar.events.append(date.unwrap().collect::<Vec<Event>>().as_mut());
        }
    }

    Some(calendar)
}

pub fn update_event(username: &String, time: &String, available: bool, date: &NaiveDate) -> EmptyResult {
    let mut event = match get_event(username, date) {
        Some(event) => event,
        None => {
            warn!("Event not found");
            return Err(());
        }
    };
    let event_dir = match get_date_event_dir(date) {
        Some(dir) => dir,
        None => {
            warn!("Failed to get user event dir");
            return Err(());
        }
    };
    event.available = available;
    event.time = time.to_string();

    match persist_entity(event_dir, username, event) {
        Ok(_) => Ok(()),
        Err(_) => Err(())
    }
}

pub fn delete_event(username: &String, date: &NaiveDate) -> EmptyResult {
    let event_dir = match get_date_event_dir(date) {
        Some(dir) => dir,
        None => {
            warn!("Failed to get user event dir");
            return Err(());
        }
    };
    match remove_file(join_paths(vec![event_dir, format!("{}.yaml", username)]).expect("Paths should be able to join")) {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("Failed to delete event {}", err);
            Err(())
        }
    }
}

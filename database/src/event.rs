use chrono::{Datelike, Days, Months, NaiveDate};
use log::warn;
use sheef_entities::{Calendar, Event};
use sheef_entities::event::CalendarDay;
use crate::{persist_entity, read_entity, read_entity_dir, validate_database_dir};
use crate::user::{get_user, get_users, user_exists};

fn validate_event_dir() -> String {
    let path = vec![validate_database_dir(), "event".to_string()].join("/");
    let result = std::fs::create_dir_all(path.as_str());
    if result.is_err() {
        panic!("Failed to create event database dir {}", result.err().unwrap());
    }

    path
}

fn get_date_event_dir(date: &NaiveDate) -> Option<String> {
    let formatted_date = &date.format("%Y-%m-%d").to_string();
    let path = vec![validate_event_dir(), formatted_date.to_string()].join("/");
    match std::fs::create_dir_all(path.as_str()) {
        Ok(_) => Some(path),
        Err(err) => {
            warn!("Failed to create event dir for date {}: {}", formatted_date, err);
            None
        }
    }
}

pub fn set_event(username: &String, time: &String, available: bool, date: &NaiveDate) -> Option<Event> {
    let user = if let Some(user) = get_user(username) {
        user.to_web_user()
    } else {
        warn!("User {} not found", username);
        return None;
    };
    let event = Event {
        username: username.clone(),
        time: time.to_string(),
        available,
        date: *date,
        user,
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

    read_entity(event_dir, username).map(|evt: Event| Event {
        username: evt.username,
        time: evt.time,
        available: evt.available,
        date: *date,
        user: get_user(username).unwrap().to_web_user(),
    })
}

pub fn get_events_for_date(date: &NaiveDate) -> Option<Vec<Event>> {
    let event_dir = match get_date_event_dir(date) {
        Some(dir) => dir,
        None => {
            warn!("Failed to get date event dir ({})", date.format("%Y-%m-%d"));
            return None;
        }
    };

    read_entity_dir(event_dir).map(|events| events.into_iter().filter(|event: &Event| user_exists(&event.username)).map(|event| {
        Event {
            username: event.username.clone(),
            user: get_user(&event.username).unwrap().to_web_user(),
            time: event.time,
            date: *date,
            available: event.available,
        }
    }).collect())
}

pub fn get_events_for_month(year: i32, month: u32) -> Option<Calendar> {
    let first_day_of_month = match NaiveDate::from_ymd_opt(year, month, 1) {
        Some(day) => day,
        None => return None
    };
    let last_day_of_month = first_day_of_month.checked_add_months(Months::new(1)).expect("One month should be able to add").checked_sub_days(Days::new(1)).expect("One day should be able to subtract");

    let mut calendar = Calendar {
        year,
        month,
        days: vec![],
    };
    (first_day_of_month.day()..last_day_of_month.day() + 1).for_each(|day| {
        let date = NaiveDate::from_ymd_opt(year, month, day).expect("Date should be valid");
        let event_dir = match get_date_event_dir(&date) {
            Some(dir) => dir,
            None => {
                warn!("Failed to get date event dir ({})", date.format("%Y-%m-%d"));
                return;
            }
        };

        let users = get_users().unwrap();
        if let Some(events) = read_entity_dir::<Event>(event_dir) {
            calendar.days.push(CalendarDay {
                events: users.map(|user| if let Some(event) = events.iter().find(|evt| evt.username == user.username.clone()) {
                    Event {
                        username: event.username.to_string(),
                        time: event.time.to_string(),
                        available: event.available,
                        date,
                        user: user.to_web_user(),
                    }
                } else {
                    Event {
                        username: user.username.clone(),
                        time: "".to_string(),
                        available: false,
                        date,
                        user: user.to_web_user(),
                    }
                }).collect(),
                date,
            });
        } else {
            calendar.days.push(CalendarDay {
                events: users.map(|user| Event {
                    username: user.username.clone(),
                    time: "".to_string(),
                    available: false,
                    date,
                    user: user.to_web_user(),
                }).collect(),
                date,
            });
        }
    });

    Some(calendar)
}

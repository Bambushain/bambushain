use chrono::{Datelike, Days, Months, NaiveDate};
use sheef_entities::{Calendar, Event};
use sheef_entities::event::CalendarDay;
use crate::{persist_entity, read_entity, read_entity_dir, validate_database_dir};
use crate::user::{get_user, get_users, user_exists};

async fn validate_event_dir() -> String {
    let path = vec![validate_database_dir().await, "event".to_string()].join("/");
    if let Err(err) = tokio::fs::create_dir_all(path.as_str()).await {
        panic!("Failed to create event database dir {err}");
    }

    path
}

async fn get_date_event_dir(date: &NaiveDate) -> Option<String> {
    let formatted_date = &date.format("%Y-%m-%d").to_string();
    let path = vec![validate_event_dir().await, formatted_date.to_string()].join("/");
    match tokio::fs::create_dir_all(path.as_str()).await {
        Ok(_) => Some(path),
        Err(err) => {
            log::warn!("Failed to create event dir for date {}: {}", formatted_date, err);
            None
        }
    }
}

pub async fn set_event(username: &String, time: &String, available: bool, date: &NaiveDate) -> Option<Event> {
    let user = if let Some(user) = get_user(username).await {
        user.to_web_user()
    } else {
        log::warn!("User {} not found", username);
        return None;
    };
    let event = Event {
        username: username.clone(),
        time: time.to_string(),
        available,
        date: *date,
        user,
    };

    let event_dir = match get_date_event_dir(date).await {
        Some(dir) => dir,
        None => {
            log::warn!("Failed to get user event dir ({})", username);
            return None;
        }
    };

    match persist_entity(event_dir, username, event).await {
        Ok(event) => Some(event),
        Err(_) => None
    }
}

pub async fn get_event(username: &String, date: &NaiveDate) -> Option<Event> {
    let event_dir = match get_date_event_dir(date).await {
        Some(dir) => dir,
        None => {
            log::warn!("Failed to get user event dir");
            return None;
        }
    };

    let user = match get_user(username).await {
        Some(user) => user,
        None => {
            log::warn!("Failed to load user");
            return None;
        }
    };

    match read_entity::<Event>(event_dir, username).await {
        Some(mut event) => {
            event.user = user.to_web_user();
            Some(event)
        }
        None => {
            log::warn!("Event not found");
            None
        }
    }
}

pub async fn get_events_for_date(date: &NaiveDate) -> Option<Vec<Event>> {
    let event_dir = match get_date_event_dir(date).await {
        Some(dir) => dir,
        None => {
            log::warn!("Failed to get date event dir ({})", date.format("%Y-%m-%d"));
            return None;
        }
    };

    match read_entity_dir::<Event>(event_dir).await {
        Some(events) => {
            let mut result = vec![];

            for mut event in events {
                if user_exists(&event.username).await {
                    event.user = get_user(&event.username).await.unwrap().to_web_user();
                    result.push(event);
                }
            }

            Some(result)
        }
        None => {
            log::warn!("Failed to read events");
            None
        }
    }
}

pub async fn get_events_for_month(year: i32, month: u32) -> Option<Calendar> {
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
    for day in first_day_of_month.day()..last_day_of_month.day() + 1 {
        let date = NaiveDate::from_ymd_opt(year, month, day).expect("Date should be valid");
        let event_dir = match get_date_event_dir(&date).await {
            Some(dir) => dir,
            None => {
                log::warn!("Failed to get date event dir ({})", date.format("%Y-%m-%d"));
                continue;
            }
        };

        let users = get_users().await.unwrap().into_iter();
        if let Some(events) = read_entity_dir::<Event>(event_dir).await {
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
    }

    Some(calendar)
}

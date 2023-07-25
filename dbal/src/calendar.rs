use chrono::{Datelike, Months, NaiveDate};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, NotSet, QueryFilter, QueryOrder};
use sea_orm::ActiveValue::Set;

use sheef_entities::{event, sheef_db_error};
use sheef_entities::prelude::*;

use crate::user::get_users;

async fn get_events_for_day(date: NaiveDate) -> SheefResult<Vec<Event>> {
    let db = open_db_connection!();

    match event::Entity::find()
        .filter(event::Column::Date.eq(date))
        .order_by_asc(event::Column::Date)
        .all(&db)
        .await {
        Ok(events) => Ok(events),
        Err(_) => Err(sheef_db_error!("calendar", "Failed to load events for date"))
    }
}

pub async fn get_events_for_month(year: i32, month: u32) -> SheefResult<Calendar> {
    let first_day_of_month = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let last_day_of_month = first_day_of_month
        .checked_add_months(Months::new(1))
        .expect("One month should be able to add");

    let users = match get_users().await {
        Ok(users) => users,
        Err(_) => return Err(sheef_db_error!("calendar", "Failed to load users"))
    };

    let mut days = vec![];

    for day in first_day_of_month.day()..last_day_of_month.day() + 1 {
        let date = NaiveDate::from_ymd_opt(year, month, day).expect("Date should be valid");
        if let Ok(events) = get_events_for_day(date).await {
            days.push(CalendarDay {
                events: users.iter().map(|user| if let Some(event) = events.iter().find(|evt| evt.username == user.username.clone()) {
                    let mut evt = event.clone();
                    evt.username = user.username.clone();
                    evt.user = user.to_web_user();
                    evt
                } else {
                    Event {
                        id: Default::default(),
                        user_id: user.id,
                        username: user.username.clone(),
                        time: Default::default(),
                        date,
                        available: false,
                        user: user.to_web_user(),
                    }
                }).collect(),
                date,
            });
        } else {
            days.push(CalendarDay {
                events: users.iter().map(|user| Event {
                    id: Default::default(),
                    user_id: user.id,
                    username: user.username.clone(),
                    time: Default::default(),
                    date,
                    available: false,
                    user: user.to_web_user(),
                }).collect(),
                date,
            });
        }
    }

    Ok(Calendar {
        month,
        year,
        days,
    })
}

pub async fn get_event(username: String, date: NaiveDate) -> SheefResult<Event> {
    let db = open_db_connection!();
    let user = get_user_by_username!(username);

    match event::Entity::find().filter(event::Column::Date.eq(date)).filter(event::Column::UserId.eq(user.id)).one(&db).await {
        Ok(Some(event)) => Ok(event),
        Ok(None) => Err(sheef_not_found_error!("event", "The event was not found")),
        Err(_) => Err(sheef_not_found_error!("event", "Failed to load event")),
    }
}

pub async fn set_event(username: String, set_event: SetEvent, date: NaiveDate) -> SheefErrorResult {
    let db = open_db_connection!();
    let user = get_user_by_username!(username.clone());

    match get_event(username.clone(), date).await {
        Ok(evt) => {
            let mut evt = evt.into_active_model();
            evt.time = Set(set_event.time.clone());
            evt.available = Set(set_event.available);
            evt.date = Set(date);
            evt
        }
        Err(_) => {
            event::ActiveModel {
                user_id: Set(user.id),
                date: Set(date),
                available: Set(set_event.available),
                time: Set(set_event.time),
                id: NotSet,
            }
        }
    }.save(&db).await.map_err(|_| sheef_db_error!("event", "Failed to create event")).map(|_| ())
}

use chrono::{Datelike, Days, Months, NaiveDate};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, JoinType, NotSet, QueryFilter, QueryOrder, QuerySelect, RelationTrait};
use sea_orm::ActiveValue::Set;

use sheef_entities::{event, sheef_db_error, user};
use sheef_entities::prelude::*;

use crate::prelude::get_user;
use crate::user::get_users;

async fn get_events_for_day(db: &DatabaseConnection, date: NaiveDate) -> SheefResult<Vec<Event>> {
    match event::Entity::find()
        .filter(event::Column::Date.eq(date))
        .order_by_asc(event::Column::Date)
        .all(db)
        .await {
        Ok(events) => Ok(events),
        Err(err) => {
            log::error!("{err}");
            Err(sheef_db_error!("calendar", "Failed to load events for date"))
        }
    }
}

pub async fn get_events_for_month(year: i32, month: u32) -> SheefResult<Calendar> {
    let first_day_of_month = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let last_day_of_month = first_day_of_month
        .checked_add_months(Months::new(1))
        .expect("One month should be able to add")
        .checked_sub_days(Days::new(1))
        .expect("Should be able to substract one day");

    let users = match get_users().await {
        Ok(users) => users,
        Err(err) => {
            log::error!("{err}");
            return Err(sheef_db_error!("calendar", "Failed to load users"));
        }
    };

    let mut days = vec![];

    let db = open_db_connection!();

    log::info!("Get calendar from {first_day_of_month} to {last_day_of_month}");

    for day in first_day_of_month.day()..last_day_of_month.day() + 1 {
        let date = NaiveDate::from_ymd_opt(year, month, day).expect("Date should be valid");
        log::info!("Get calendar for date {date}");
        if let Ok(events) = get_events_for_day(&db, date).await {
            days.push(CalendarDay {
                events: users.iter().map(|user| if let Some(event) = events.iter().find(|evt| evt.user_id == user.id) {
                    log::info!("Found event {event:?}");
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

    let _ = db.close().await;

    Ok(Calendar {
        month,
        year,
        days,
    })
}

pub async fn get_event(username: String, date: NaiveDate) -> SheefResult<Event> {
    let db = open_db_connection!();

    let result = match event::Entity::find()
        .filter(event::Column::Date.eq(date))
        .filter(user::Column::Username.eq(username))
        .join(JoinType::InnerJoin, event::Relation::User.def())
        .one(&db)
        .await {
        Ok(Some(event)) => Ok(event),
        Ok(None) => Err(sheef_not_found_error!("event", "The event was not found")),
        Err(err) => {
            log::error!("{err}");
            Err(sheef_not_found_error!("event", "Failed to load event"))
        }
    };

    let _ = db.close().await;

    result
}

pub async fn set_event(username: String, set_event: SetEvent, date: NaiveDate) -> SheefErrorResult {
    let db = open_db_connection!();
    let user = match get_user(username.clone()).await {
        Ok(user) => user,
        Err(err) => return Err(err)
    };

    let result = match get_event(username.clone(), date).await {
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
    }
        .save(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            sheef_db_error!("event", "Failed to create event")
        })
        .map(|_| ());

    let _ = db.close().await;

    result
}

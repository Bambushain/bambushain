use date_range::DateRange;
use sea_orm::prelude::*;
use sea_orm::{Condition, IntoActiveModel, JoinType, NotSet, QueryOrder, QuerySelect, Set};

use bamboo_common_core::entities::event;
use bamboo_common_core::entities::event::GroveEvent;
use bamboo_common_core::entities::*;
use bamboo_common_core::error::*;

pub async fn get_events(
    range: DateRange,
    user_id: i32,
    grove_id: Option<i32>,
    db: &DatabaseConnection,
) -> BambooResult<Vec<GroveEvent>> {
    let query = event::Entity::find()
        .distinct_on(vec![event::Column::Id])
        .join_rev(
            JoinType::LeftJoin,
            grove_user::Entity::belongs_to(event::Entity)
                .from(grove_user::Column::GroveId)
                .to(event::Column::GroveId)
                .into(),
        )
        .filter(
            Condition::any()
                .add(event::Column::StartDate.between(range.since(), range.until()))
                .add(event::Column::EndDate.between(range.since(), range.until())),
        )
        .filter(
            Condition::any()
                .add(
                    Condition::all()
                        .add(event::Column::IsPrivate.eq(true))
                        .add(event::Column::UserId.eq(user_id)),
                )
                .add(
                    Condition::all()
                        .add(event::Column::IsPrivate.eq(false))
                        .add(grove_user::Column::UserId.eq(user_id)),
                ),
        );
    let events = if let Some(grove_id) = grove_id {
        query.filter(event::Column::GroveId.eq(grove_id))
    } else {
        query
    }
    .order_by_asc(event::Column::Id)
    .all(db)
    .await
    .map_err(|err| {
        log::error!("Failed to load events {err}");
        BambooError::database("event", "Failed to load events")
    })?;

    let users = super::get_users(user_id, db).await?;
    let groves = super::get_all_groves(db).await?;

    Ok(events
        .iter()
        .map(|event| {
            let user = if let Some(user_id) = event.user_id {
                users.iter().find_map(|user| {
                    if user.id == user_id {
                        Some(user.clone())
                    } else {
                        None
                    }
                })
            } else {
                None
            };
            let grove = groves
                .iter()
                .find(|grove| grove.id == event.grove_id.unwrap_or(-1))
                .cloned();

            GroveEvent::from_event(event.clone(), user, grove)
        })
        .collect())
}

pub async fn get_event(id: i32, user_id: i32, db: &DatabaseConnection) -> BambooResult<Event> {
    event::Entity::find_by_id(id)
        .join_rev(
            JoinType::LeftJoin,
            grove_user::Entity::belongs_to(event::Entity)
                .from(grove_user::Column::GroveId)
                .to(event::Column::GroveId)
                .into(),
        )
        .filter(grove_user::Column::UserId.eq(user_id))
        .filter(
            Condition::any()
                .add(event::Column::IsPrivate.eq(false))
                .add(
                    Condition::all()
                        .add(event::Column::IsPrivate.eq(true))
                        .add(event::Column::UserId.eq(user_id)),
                ),
        )
        .one(db)
        .await
        .map_err(|err| {
            log::error!("Failed to load events {err}");
            BambooError::database("event", "Failed to load events")
        })
        .map(|data| {
            if let Some(data) = data {
                Ok(data)
            } else {
                Err(BambooError::not_found("event", "The event was not found"))
            }
        })?
}

pub async fn create_event(
    event: GroveEvent,
    user_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<Event> {
    let mut model = event.to_event().into_active_model();
    model.id = NotSet;
    model.user_id = Set(Some(user_id));

    model.insert(db).await.map_err(|err| {
        log::error!("Failed to create event {err}");
        BambooError::database("event", "Failed to create event")
    })
}

pub async fn update_event(
    user_id: i32,
    id: i32,
    event: GroveEvent,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    event::Entity::update_many()
        .filter(event::Column::Id.eq(id))
        .filter(event::Column::UserId.eq(user_id))
        .col_expr(event::Column::StartDate, Expr::value(event.start_date))
        .col_expr(event::Column::EndDate, Expr::value(event.end_date))
        .col_expr(event::Column::Description, Expr::value(event.description))
        .col_expr(event::Column::Title, Expr::value(event.title))
        .col_expr(event::Column::Color, Expr::value(event.color))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("Failed to update event {err}");
            BambooError::database("event", "Failed to update event")
        })
        .map(|_| ())
}

pub async fn delete_event(user_id: i32, id: i32, db: &DatabaseConnection) -> BambooErrorResult {
    event::Entity::delete_many()
        .filter(event::Column::Id.eq(id))
        .filter(event::Column::UserId.eq(user_id))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("Failed to delete event {err}");
            BambooError::database("event", "Failed to delete event")
        })
        .map(|_| ())
}

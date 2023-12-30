use date_range::DateRange;
use sea_orm::prelude::*;
use sea_orm::{Condition, IntoActiveModel, NotSet, QueryOrder, Set};

use bamboo_entities::event;
use bamboo_entities::prelude::*;
use bamboo_error::*;

pub async fn get_events(
    grove_id: i32,
    range: DateRange,
    user_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<Vec<Event>> {
    event::Entity::find()
        .filter(event::Column::GroveId.eq(grove_id))
        .filter(
            Condition::any()
                .add(
                    Condition::all()
                        .add(event::Column::StartDate.gte(range.since()))
                        .add(event::Column::StartDate.lte(range.until())),
                )
                .add(
                    Condition::all()
                        .add(event::Column::EndDate.gte(range.since()))
                        .add(event::Column::EndDate.lte(range.until())),
                ),
        )
        .filter(
            Condition::any()
                .add(event::Column::IsPrivate.eq(false))
                .add(
                    Condition::all()
                        .add(event::Column::IsPrivate.eq(true))
                        .add(event::Column::UserId.eq(user_id)),
                ),
        )
        .order_by_asc(event::Column::Id)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("Failed to load events {err}");
            BambooError::database("event", "Failed to load events")
        })
}

pub async fn get_event(
    id: i32,
    grove_id: i32,
    user_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<Event> {
    event::Entity::find_by_id(id)
        .filter(event::Column::GroveId.eq(grove_id))
        .one(db)
        .await
        .map_err(|err| {
            log::error!("Failed to load events {err}");
            BambooError::database("event", "Failed to load events")
        })
        .map(|data| {
            if let Some(data) = data {
                if data.is_private && data.user_id != Some(user_id) {
                    Err(BambooError::not_found("event", "The event was not found"))
                } else {
                    Ok(data)
                }
            } else {
                Err(BambooError::not_found("event", "The event was not found"))
            }
        })?
}

pub async fn create_event(
    event: Event,
    grove_id: i32,
    user_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<Event> {
    let mut model = event.clone().into_active_model();
    model.id = NotSet;
    model.grove_id = Set(grove_id);
    if event.is_private {
        model.user_id = Set(Some(user_id));
    }

    model.insert(db).await.map_err(|err| {
        log::error!("Failed to create event {err}");
        BambooError::database("event", "Failed to create event")
    })
}

pub async fn update_event(
    grove_id: i32,
    id: i32,
    event: Event,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    event::Entity::update_many()
        .filter(event::Column::Id.eq(id))
        .filter(event::Column::GroveId.eq(grove_id))
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

pub async fn delete_event(grove_id: i32, id: i32, db: &DatabaseConnection) -> BambooErrorResult {
    event::Entity::delete_many()
        .filter(event::Column::Id.eq(id))
        .filter(event::Column::GroveId.eq(grove_id))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("Failed to delete event {err}");
            BambooError::database("event", "Failed to delete event")
        })
        .map(|_| ())
}

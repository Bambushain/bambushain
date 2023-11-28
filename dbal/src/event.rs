use date_range::DateRange;
use sea_orm::prelude::*;
use sea_orm::{Condition, IntoActiveModel, NotSet, QueryOrder};

use bamboo_entities::event;
use bamboo_entities::prelude::*;

pub async fn get_events(range: DateRange, db: &DatabaseConnection) -> PandaPartyResult<Vec<Event>> {
    event::Entity::find()
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
        .order_by_asc(event::Column::Id)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            bamboo_db_error!("event", "Failed to load events")
        })
}

pub async fn create_event(event: Event, db: &DatabaseConnection) -> PandaPartyResult<Event> {
    let mut model = event.into_active_model();
    model.id = NotSet;

    model.insert(db).await.map_err(|err| {
        log::error!("{err}");
        bamboo_db_error!("event", "Failed to create event")
    })
}

pub async fn update_event(id: i32, event: Event, db: &DatabaseConnection) -> PandaPartyErrorResult {
    event::Entity::update_many()
        .filter(event::Column::Id.eq(id))
        .col_expr(event::Column::StartDate, Expr::value(event.start_date))
        .col_expr(event::Column::EndDate, Expr::value(event.end_date))
        .col_expr(event::Column::Description, Expr::value(event.description))
        .col_expr(event::Column::Title, Expr::value(event.title))
        .col_expr(event::Column::Color, Expr::value(event.color))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            bamboo_db_error!("event", "Failed to update event")
        })
        .map(|_| ())
}

pub async fn delete_event(id: i32, db: &DatabaseConnection) -> PandaPartyErrorResult {
    event::Entity::delete_many()
        .filter(event::Column::Id.eq(id))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            bamboo_db_error!("event", "Failed to delete event")
        })
        .map(|_| ())
}

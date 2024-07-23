use bamboo_common_core::entities::event;
use bamboo_common_core::entities::event::GroveEvent;
use bamboo_common_core::entities::user::WebUser;
use bamboo_common_core::entities::*;
use bamboo_common_core::error::*;
use chrono::NaiveDate;
use date_range::DateRange;
use sea_orm::prelude::*;
use sea_orm::sea_query::IntoCondition;
use sea_orm::{
    Condition, FromQueryResult, IntoActiveModel, JoinType, NotSet, QueryOrder, QuerySelect,
    SelectModel, Selector, Set,
};
use serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default, FromQueryResult,
)]
#[serde(rename_all = "camelCase")]
struct LoadEvent {
    pub event_id: i32,
    pub title: String,
    pub description: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub color: String,
    pub is_private: bool,
    pub user_id: Option<i32>,
    pub display_name: Option<String>,
    pub discord_name: Option<String>,
    pub email: Option<String>,
    pub grove_id: Option<i32>,
    pub grove_name: Option<String>,
}

impl From<LoadEvent> for GroveEvent {
    fn from(value: LoadEvent) -> Self {
        let user = if let Some(id) = value.user_id {
            Some(WebUser {
                id,
                email: value.email.unwrap(),
                display_name: value.display_name.unwrap(),
                discord_name: value.discord_name.unwrap_or("".to_string()),
            })
        } else {
            None
        };
        let grove = if let Some(id) = value.grove_id {
            Some(Grove {
                id,
                name: value.grove_name.unwrap(),
            })
        } else {
            None
        };

        GroveEvent {
            id: value.event_id,
            title: value.title,
            description: value.description,
            start_date: value.start_date,
            end_date: value.end_date,
            color: value.color,
            is_private: value.is_private,
            user,
            grove,
        }
    }
}

fn get_event_query(
    user_id: i32,
    additional_filter: impl IntoCondition,
) -> Selector<SelectModel<LoadEvent>> {
    event::Entity::find()
        .select_only()
        .column_as(event::Column::Id, "event_id")
        .column_as(event::Column::Title, "title")
        .column_as(event::Column::Description, "description")
        .column_as(event::Column::StartDate, "start_date")
        .column_as(event::Column::EndDate, "end_date")
        .column_as(event::Column::Color, "color")
        .column_as(event::Column::IsPrivate, "is_private")
        .column_as(user::Column::Id, "user_id")
        .column_as(user::Column::DisplayName, "display_name")
        .column_as(user::Column::DiscordName, "discord_name")
        .column_as(user::Column::Email, "email")
        .column_as(grove::Column::Id, "grove_id")
        .column_as(grove::Column::Name, "grove_name")
        .join_rev(
            JoinType::LeftJoin,
            grove_user::Entity::belongs_to(event::Entity)
                .from(grove_user::Column::GroveId)
                .to(event::Column::GroveId)
                .on_condition(|gu, e| {
                    Condition::all()
                        .add(
                            Expr::col((gu.clone(), grove_user::Column::GroveId))
                                .eq(Expr::col((e.clone(), event::Column::GroveId))),
                        )
                        .add(
                            Condition::any()
                                .add(
                                    Expr::col((gu, grove_user::Column::UserId))
                                        .eq(Expr::col((e.clone(), event::Column::UserId))),
                                )
                                .add(Expr::col((e, event::Column::UserId)).is_null()),
                        )
                })
                .into(),
        )
        .join_rev(
            JoinType::LeftJoin,
            grove::Entity::belongs_to(grove_user::Entity)
                .from(grove::Column::Id)
                .to(grove_user::Column::GroveId)
                .into(),
        )
        .join_rev(
            JoinType::LeftJoin,
            user::Entity::belongs_to(grove_user::Entity)
                .from(user::Column::Id)
                .to(grove_user::Column::UserId)
                .into(),
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
        )
        .filter(additional_filter)
        .order_by_asc(event::Column::Id)
        .into_model::<LoadEvent>()
}

pub async fn get_events(
    range: DateRange,
    user_id: i32,
    grove_id: Option<i32>,
    db: &DatabaseConnection,
) -> BambooResult<Vec<GroveEvent>> {
    let additional_filter = Condition::any()
        .add(event::Column::StartDate.between(range.since(), range.until()))
        .add(event::Column::EndDate.between(range.since(), range.until()));

    get_event_query(
        user_id,
        if let Some(grove_id) = grove_id {
            Condition::all()
                .add(additional_filter)
                .add(event::Column::GroveId.eq(grove_id))
        } else {
            additional_filter
        },
    )
    .all(db)
    .await
    .map_err(|err| {
        log::error!("Failed to load events {err}");
        BambooError::database("event", "Failed to load events")
    })
    .map(|events| {
        events
            .iter()
            .cloned()
            .map(|event| event.into())
            .collect::<Vec<GroveEvent>>()
    })
}

pub async fn get_event(id: i32, user_id: i32, db: &DatabaseConnection) -> BambooResult<GroveEvent> {
    get_event_query(
        user_id,
        Condition::all()
            .add(grove_user::Column::UserId.eq(user_id))
            .add(event::Column::Id.eq(id)),
    )
    .one(db)
    .await
    .map_err(|err| {
        log::error!("Failed to load events {err}");
        BambooError::database("event", "Failed to load events")
    })
    .map(|data| {
        if let Some(data) = data {
            Ok(data.into())
        } else {
            Err(BambooError::not_found("event", "The event was not found"))
        }
    })?
}

pub async fn create_event(
    event: GroveEvent,
    user_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<GroveEvent> {
    let mut model = event.to_event().into_active_model();
    model.id = NotSet;
    model.user_id = Set(Some(user_id));

    let created = model.insert(db).await.map_err(|err| {
        log::error!("Failed to create event {err}");
        BambooError::database("event", "Failed to create event")
    })?;

    let event_id = created.id;
    let event = get_event(event_id, user_id, db).await?;

    Ok(event)
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

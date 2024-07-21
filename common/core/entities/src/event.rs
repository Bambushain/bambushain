use std::str::FromStr;

use chrono::NaiveDate;
use color_art::{color, Color};
#[cfg(feature = "backend")]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{Event, Grove, User};
#[cfg(feature = "backend")]
use bamboo_common_backend_macros::*;

fn set_false() -> bool {
    false
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[cfg_attr(
    feature = "backend",
    derive(DeriveEntityModel, Responder),
    sea_orm(table_name = "event", schema_name = "bamboo")
)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[cfg_attr(feature = "backend", sea_orm(primary_key))]
    #[serde(default)]
    pub id: i32,
    pub title: String,
    pub description: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub color: String,
    #[serde(default = "set_false")]
    pub is_private: bool,
    #[serde(skip)]
    pub user_id: Option<i32>,
    #[serde(skip)]
    pub grove_id: i32,
}

#[cfg(feature = "backend")]
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::grove::Entity",
        from = "Column::GroveId",
        to = "super::grove::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Grove,
}

#[cfg(feature = "backend")]
impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

#[cfg(feature = "backend")]
impl Related<super::grove::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Grove.def()
    }
}

#[cfg(feature = "backend")]
impl ActiveModelBehavior for ActiveModel {}

impl Model {
    #[cfg(feature = "frontend")]
    pub fn new(
        title: String,
        description: String,
        start_date: NaiveDate,
        end_date: NaiveDate,
        color: Color,
        is_private: bool,
        grove_id: i32,
    ) -> Self {
        Self {
            id: i32::default(),
            title,
            description,
            start_date,
            end_date,
            color: color.hex(),
            is_private,
            user_id: None,
            grove_id,
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color.hex();
    }

    pub fn color(&self) -> Color {
        Color::from_str(self.color.as_str()).unwrap_or(color!(#9f2637))
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[cfg_attr(feature = "backend", derive(Responder))]
#[serde(rename_all = "camelCase")]
pub struct GroveEvent {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub color: String,
    pub is_private: bool,
    pub user: Option<User>,
    pub grove: Grove,
}

impl GroveEvent {
    pub fn to_event(&self) -> Event {
        Event {
            id: self.id,
            title: self.title.clone(),
            description: self.description.clone(),
            start_date: self.start_date,
            end_date: self.end_date,
            color: self.title.clone(),
            is_private: self.is_private,
            user_id: self.user.clone().map(|user| user.id),
            grove_id: self.grove.id,
        }
    }

    pub fn from_event(event: Event, user: Option<User>, grove: Grove) -> Self {
        GroveEvent {
            id: event.id,
            title: event.title,
            description: event.description,
            start_date: event.start_date,
            end_date: event.end_date,
            color: event.color,
            is_private: event.is_private,
            user,
            grove,
        }
    }
}

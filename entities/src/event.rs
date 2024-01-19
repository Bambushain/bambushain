use std::str::FromStr;

use chrono::NaiveDate;
use color_art::{color, Color};
#[cfg(not(target_arch = "wasm32"))]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
use bamboo_macros::*;

fn set_false() -> bool {
    false
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(DeriveEntityModel, Responder),
    sea_orm(table_name = "event", schema_name = "bamboo")
)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[cfg_attr(not(target_arch = "wasm32"), sea_orm(primary_key))]
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
    #[cfg(not(target_arch = "wasm32"))]
    #[serde(skip)]
    pub grove_id: i32,
}

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(not(target_arch = "wasm32"))]
impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Related<super::grove::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Grove.def()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl ActiveModelBehavior for ActiveModel {}

impl Model {
    #[cfg(target_arch = "wasm32")]
    pub fn new(
        title: String,
        description: String,
        start_date: NaiveDate,
        end_date: NaiveDate,
        color: Color,
        is_private: bool,
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
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color.hex();
    }

    pub fn color(&self) -> Color {
        Color::from_str(self.color.as_str()).unwrap_or(color!(#9f2637))
    }
}

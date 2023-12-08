use std::str::FromStr;

use chrono::NaiveDate;
use color_art::{color, Color};
#[cfg(not(target_arch = "wasm32"))]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(DeriveEntityModel),
    sea_orm(table_name = "event", schema_name = "bamboo")
)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[cfg_attr(not(target_arch = "wasm32"), sea_orm(primary_key))]
    pub id: i32,
    pub title: String,
    pub description: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub color: String,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[cfg(not(target_arch = "wasm32"))]
impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn new(
        title: String,
        description: String,
        start_date: NaiveDate,
        end_date: NaiveDate,
        color: Color,
    ) -> Self {
        Self {
            id: i32::default(),
            title,
            description,
            start_date,
            end_date,
            color: color.hex(),
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color.hex();
    }

    pub fn color(&self) -> Color {
        Color::from_str(self.color.as_str()).unwrap_or(color!(#9f2637))
    }
}

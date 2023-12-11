#[cfg(not(target_arch = "wasm32"))]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BTreeSet;

#[cfg(not(target_arch = "wasm32"))]
use bamboo_macros::*;

use crate::prelude::CustomCharacterFieldOption;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
pub struct CustomField {
    pub values: BTreeSet<String>,
    pub label: String,
    pub position: usize,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(DeriveEntityModel, Responder),
    sea_orm(table_name = "custom_character_field", schema_name = "final_fantasy")
)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[cfg_attr(not(target_arch = "wasm32"), sea_orm(primary_key))]
    pub id: i32,
    pub label: String,
    #[cfg(not(target_arch = "wasm32"))]
    #[serde(skip)]
    pub user_id: i32,
    pub position: i32,
    #[cfg_attr(not(target_arch = "wasm32"), sea_orm(ignore))]
    pub options: Vec<CustomCharacterFieldOption>,
}

impl PartialOrd for Model {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Model {
    fn cmp(&self, other: &Self) -> Ordering {
        self.position.cmp(&other.position)
    }
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
    #[sea_orm(has_many = "super::custom_character_field_option::Entity")]
    CustomFieldOption,
    #[sea_orm(has_many = "super::custom_character_field_value::Entity")]
    CustomFieldValue,
}

#[cfg(not(target_arch = "wasm32"))]
impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Related<super::custom_character_field_option::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CustomFieldOption.def()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Related<super::custom_character_field_value::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CustomFieldValue.def()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn new(label: String, options: Vec<CustomCharacterFieldOption>) -> Self {
        Self {
            id: i32::default(),
            label,
            #[cfg(not(target_arch = "wasm32"))]
            user_id: i32::default(),
            options,
            position: 0,
        }
    }
}

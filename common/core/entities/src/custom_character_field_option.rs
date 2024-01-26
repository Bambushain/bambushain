use std::cmp::Ordering;

#[cfg(feature = "backend")]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "backend")]
use bamboo_common_backend_macros::*;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "backend",
    derive(DeriveEntityModel, Responder),
    sea_orm(
        table_name = "custom_character_field_option",
        schema_name = "final_fantasy"
    )
)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[cfg_attr(feature = "backend", sea_orm(primary_key))]
    #[serde(default)]
    pub id: i32,
    pub label: String,
    #[serde(skip)]
    pub custom_character_field_id: i32,
}

impl PartialOrd for Model {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Model {
    fn cmp(&self, other: &Self) -> Ordering {
        self.label.cmp(&other.label)
    }
}

#[cfg(feature = "backend")]
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::custom_character_field::Entity",
        from = "Column::CustomCharacterFieldId",
        to = "super::custom_character_field::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    CustomCharacterField,
}

#[cfg(feature = "backend")]
impl Related<super::custom_character_field::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CustomCharacterField.def()
    }
}

#[cfg(feature = "backend")]
impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn new(label: String, custom_character_field_id: i32) -> Self {
        Self {
            id: i32::default(),
            label,
            custom_character_field_id,
        }
    }
}

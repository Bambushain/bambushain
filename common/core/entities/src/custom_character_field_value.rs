#[cfg(feature = "backend")]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "backend")]
use bamboo_common_backend_macros::*;

use crate::{CustomCharacterField, CustomCharacterFieldOption};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[cfg_attr(
    feature = "backend",
    derive(DeriveEntityModel, Responder),
    sea_orm(
        table_name = "custom_character_field_value",
        schema_name = "final_fantasy"
    )
)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[cfg_attr(feature = "backend", sea_orm(primary_key))]
    #[serde(default)]
    pub id: i32,
    pub character_id: i32,
    pub custom_character_field_id: i32,
    pub custom_character_field_option_id: i32,
    #[cfg_attr(feature = "backend", sea_orm(ignore))]
    pub custom_character_field: CustomCharacterField,
    #[cfg_attr(feature = "backend", sea_orm(ignore))]
    pub custom_character_field_option: CustomCharacterFieldOption,
}

#[cfg(feature = "backend")]
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::character::Entity",
        from = "Column::CharacterId",
        to = "super::character::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Character,
    #[sea_orm(
        belongs_to = "super::custom_character_field::Entity",
        from = "Column::CustomCharacterFieldId",
        to = "super::custom_character_field::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    CustomCharacterField,
    #[sea_orm(
        belongs_to = "super::custom_character_field_option::Entity",
        from = "Column::CustomCharacterFieldOptionId",
        to = "super::custom_character_field_option::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    CustomCharacterFieldOption,
}

#[cfg(feature = "backend")]
impl Related<super::character::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Character.def()
    }
}

#[cfg(feature = "backend")]
impl Related<super::custom_character_field::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CustomCharacterField.def()
    }
}

#[cfg(feature = "backend")]
impl Related<super::custom_character_field_option::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CustomCharacterFieldOption.def()
    }
}

#[cfg(feature = "backend")]
impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn new(
        character_id: i32,
        custom_character_field: CustomCharacterField,
        custom_character_field_option: CustomCharacterFieldOption,
    ) -> Self {
        Self {
            id: i32::default(),
            character_id,
            custom_character_field_id: custom_character_field.id,
            custom_character_field_option_id: custom_character_field_option.id,
            custom_character_field,
            custom_character_field_option,
        }
    }
}

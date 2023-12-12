use std::cmp::Ordering;

#[cfg(not(target_arch = "wasm32"))]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(not(not(target_arch = "wasm32")))]
use strum_macros::EnumIter;

#[cfg(not(target_arch = "wasm32"))]
use bamboo_macros::*;

#[derive(Serialize, Deserialize, EnumIter, Debug, Eq, PartialEq, Clone, Default, Copy)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(DeriveActiveEnum),
    sea_orm(
        rs_type = "String",
        db_type = "Enum",
        enum_name = "final_fantasy.housing_district"
    )
)]
pub enum HousingDistrict {
    #[default]
    #[cfg_attr(
        not(target_arch = "wasm32"),
        sea_orm(string_value = "the_lavender_beds")
    )]
    TheLavenderBeds,
    #[cfg_attr(not(target_arch = "wasm32"), sea_orm(string_value = "mist"))]
    Mist,
    #[cfg_attr(not(target_arch = "wasm32"), sea_orm(string_value = "the_goblet"))]
    TheGoblet,
    #[cfg_attr(not(target_arch = "wasm32"), sea_orm(string_value = "shirogane"))]
    Shirogane,
    #[cfg_attr(not(target_arch = "wasm32"), sea_orm(string_value = "empyreum"))]
    Empyreum,
}

impl HousingDistrict {
    pub fn get_name(self) -> String {
        match self {
            HousingDistrict::TheLavenderBeds => "the_lavender_beds",
            HousingDistrict::Mist => "mist",
            HousingDistrict::TheGoblet => "the_goblet",
            HousingDistrict::Shirogane => "shirogane",
            HousingDistrict::Empyreum => "empyreum",
        }
        .to_string()
    }
}

impl ToString for HousingDistrict {
    fn to_string(&self) -> String {
        match self {
            HousingDistrict::TheLavenderBeds => "Lavender Beete",
            HousingDistrict::Mist => "Dorf des Nebels",
            HousingDistrict::TheGoblet => "Kelchkuppe",
            HousingDistrict::Shirogane => "Shirogane",
            HousingDistrict::Empyreum => "Empyreum",
        }
        .to_string()
    }
}

impl From<String> for HousingDistrict {
    fn from(value: String) -> Self {
        match value.as_str() {
            "the_lavender_beds" => HousingDistrict::TheLavenderBeds,
            "mist" => HousingDistrict::Mist,
            "the_goblet" => HousingDistrict::TheGoblet,
            "shirogane" => HousingDistrict::Shirogane,
            "empyreum" => HousingDistrict::Empyreum,
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for HousingDistrict {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HousingDistrict {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_name().cmp(&other.get_name())
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(DeriveEntityModel, Responder),
    sea_orm(table_name = "character_housing", schema_name = "final_fantasy")
)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[cfg_attr(not(target_arch = "wasm32"), sea_orm(primary_key))]
    pub id: i32,
    pub district: HousingDistrict,
    pub ward: u8,
    pub plot: u8,
    pub character_id: i32,
}

impl PartialOrd for Model {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Model {
    fn cmp(&self, other: &Self) -> Ordering {
        self.district
            .cmp(&other.district)
            .then(self.ward.cmp(&other.ward))
            .then(self.plot.cmp(&other.plot))
    }
}

#[cfg(not(target_arch = "wasm32"))]
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
}

#[cfg(not(target_arch = "wasm32"))]
impl Related<super::character::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Character.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn new(character_id: i32, district: HousingDistrict, ward: u8, plot: u8) -> Self {
        Self {
            id: i32::default(),
            district,
            ward,
            plot,
            character_id,
        }
    }
}

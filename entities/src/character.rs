use std::cmp::Ordering;

#[cfg(feature = "backend")]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(not(feature = "backend"))]
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, EnumIter, Debug, Eq, PartialEq, Clone, Default, Copy)]
#[cfg_attr(feature = "backend", derive(DeriveActiveEnum), sea_orm(rs_type = "String", db_type = "Enum", enum_name = "final_fantasy.character_race"))]
pub enum CharacterRace {
    #[default]
    #[cfg_attr(feature = "backend", sea_orm(string_value = "hyur"))]
    Hyur,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "elezen"))]
    Elezen,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "lalafell"))]
    Lalafell,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "miqote"))]
    Miqote,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "roegadyn"))]
    Roegadyn,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "au_ra"))]
    AuRa,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "hrothgar"))]
    Hrothgar,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "viera"))]
    Viera,
}

impl CharacterRace {
    pub fn get_race_name(self) -> String {
        match self {
            Self::Hyur => "hyur",
            Self::Elezen => "elezen",
            Self::Lalafell => "lalafell",
            Self::Miqote => "miqote",
            Self::Roegadyn => "roegadyn",
            Self::AuRa => "au_ra",
            Self::Hrothgar => "hrothgar",
            Self::Viera => "viera",
        }.to_string()
    }
}

impl PartialOrd for CharacterRace {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_string().partial_cmp(&other.to_string())
    }
}

impl Ord for CharacterRace {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_string().cmp(&other.to_string())
    }
}

impl ToString for CharacterRace {
    fn to_string(&self) -> String {
        match self {
            Self::Hyur => "Hyuran",
            Self::Elezen => "Elezen",
            Self::Lalafell => "Lalafell",
            Self::Miqote => "Miqo'te",
            Self::Roegadyn => "Roegadyn",
            Self::AuRa => "Au Ra",
            Self::Hrothgar => "Hrothgar",
            Self::Viera => "Viera",
        }.to_string()
    }
}

impl From<String> for CharacterRace {
    fn from(value: String) -> Self {
        match value.as_str() {
            "hyur" => Self::Hyur,
            "elezen" => Self::Elezen,
            "lalafell" => Self::Lalafell,
            "miqote" => Self::Miqote,
            "roegadyn" => Self::Roegadyn,
            "au_ra" => Self::AuRa,
            "hrothgar" => Self::Hrothgar,
            "viera" => Self::Viera,
            _ => unreachable!()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[cfg_attr(feature = "backend", derive(DeriveEntityModel), sea_orm(table_name = "character", schema_name = "final_fantasy"))]
pub struct Model {
    #[cfg_attr(feature = "backend", sea_orm(primary_key))]
    pub id: i32,
    pub race: CharacterRace,
    pub name: String,
    pub world: String,
    #[cfg(feature = "backend")]
    #[serde(skip)]
    pub user_id: i32,
}

#[cfg(feature = "backend")]
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::user::Entity", from = "Column::UserId", to = "super::user::Column::Id", on_update = "Cascade", on_delete = "Cascade")]
    User,
}

#[cfg(feature = "backend")]
impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

#[cfg(feature = "backend")]
impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn new(race: CharacterRace, name: String, world: String) -> Self {
        Self {
            id: i32::default(),
            race,
            name,
            world,
            #[cfg(feature = "backend")]
            user_id: i32::default(),
        }
    }
}

use std::cmp::Ordering;

#[cfg(feature = "backend")]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(not(feature = "backend"))]
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, EnumIter, Debug, Eq, PartialEq, Clone, Default, Copy)]
#[cfg_attr(feature = "backend", derive(DeriveActiveEnum), sea_orm(rs_type = "String", db_type = "Enum", enum_name = "final_fantasy.fighter_job"))]
pub enum FighterJob {
    #[default]
    #[cfg_attr(feature = "backend", sea_orm(string_value = "paladin"))]
    Paladin,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "warrior"))]
    Warrior,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "dark_knight"))]
    DarkKnight,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "gunbreaker"))]
    Gunbreaker,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "white_mage"))]
    WhiteMage,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "scholar"))]
    Scholar,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "astrologian"))]
    Astrologian,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "sage"))]
    Sage,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "monk"))]
    Monk,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "dragoon"))]
    Dragoon,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "ninja"))]
    Ninja,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "samurai"))]
    Samurai,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "reaper"))]
    Reaper,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "bard"))]
    Bard,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "machinist"))]
    Machinist,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "dancer"))]
    Dancer,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "black_mage"))]
    BlackMage,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "summoner"))]
    Summoner,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "red_mage"))]
    RedMage,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "blue_mage"))]
    BlueMage,
}

impl FighterJob {
    pub fn get_file_name(self) -> String {
        match self {
            FighterJob::Paladin => "paladin.png",
            FighterJob::Warrior => "warrior.png",
            FighterJob::DarkKnight => "darkknight.png",
            FighterJob::Gunbreaker => "gunbreaker.png",
            FighterJob::WhiteMage => "whitemage.png",
            FighterJob::Scholar => "scholar.png",
            FighterJob::Astrologian => "astrologian.png",
            FighterJob::Sage => "sage.png",
            FighterJob::Monk => "monk.png",
            FighterJob::Dragoon => "dragoon.png",
            FighterJob::Ninja => "ninja.png",
            FighterJob::Samurai => "samurai.png",
            FighterJob::Reaper => "reaper.png",
            FighterJob::Bard => "bard.png",
            FighterJob::Machinist => "machinist.png",
            FighterJob::Dancer => "dancer.png",
            FighterJob::BlackMage => "blackmage.png",
            FighterJob::Summoner => "summoner.png",
            FighterJob::RedMage => "redmage.png",
            FighterJob::BlueMage => "bluemage.png",
        }.to_string()
    }

    pub fn get_job_name(self) -> String {
        match self {
            FighterJob::Paladin => "Paladin",
            FighterJob::Warrior => "Warrior",
            FighterJob::DarkKnight => "DarkKnight",
            FighterJob::Gunbreaker => "Gunbreaker",
            FighterJob::WhiteMage => "WhiteMage",
            FighterJob::Scholar => "Scholar",
            FighterJob::Astrologian => "Astrologian",
            FighterJob::Sage => "Sage",
            FighterJob::Monk => "Monk",
            FighterJob::Dragoon => "Dragoon",
            FighterJob::Ninja => "Ninja",
            FighterJob::Samurai => "Samurai",
            FighterJob::Reaper => "Reaper",
            FighterJob::Bard => "Bard",
            FighterJob::Machinist => "Machinist",
            FighterJob::Dancer => "Dancer",
            FighterJob::BlackMage => "BlackMage",
            FighterJob::Summoner => "Summoner",
            FighterJob::RedMage => "RedMage",
            FighterJob::BlueMage => "BlueMage",
        }.to_string()
    }
}

impl PartialOrd for FighterJob {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_string().partial_cmp(&other.to_string())
    }
}

impl Ord for FighterJob {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_string().cmp(&other.to_string())
    }
}

impl ToString for FighterJob {
    fn to_string(&self) -> String {
        match self {
            FighterJob::Paladin => "Paladin",
            FighterJob::Warrior => "Krieger",
            FighterJob::DarkKnight => "Dunkelritter",
            FighterJob::Gunbreaker => "Revolverklinge",
            FighterJob::WhiteMage => "Weißmagier",
            FighterJob::Scholar => "Gelehrter",
            FighterJob::Astrologian => "Astrologe",
            FighterJob::Sage => "Weiser",
            FighterJob::Monk => "Mönch",
            FighterJob::Dragoon => "Dragoon",
            FighterJob::Ninja => "Ninja",
            FighterJob::Samurai => "Samurai",
            FighterJob::Reaper => "Schnitter",
            FighterJob::Bard => "Barde",
            FighterJob::Machinist => "Maschinist",
            FighterJob::Dancer => "Tänzer",
            FighterJob::BlackMage => "Schwarzmagier",
            FighterJob::Summoner => "Beschwörer",
            FighterJob::RedMage => "Rotmagier",
            FighterJob::BlueMage => "Blaumagier",
        }.to_string()
    }
}

impl From<String> for FighterJob {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Paladin" => FighterJob::Paladin,
            "Warrior" => FighterJob::Warrior,
            "DarkKnight" => FighterJob::DarkKnight,
            "Gunbreaker" => FighterJob::Gunbreaker,
            "WhiteMage" => FighterJob::WhiteMage,
            "Scholar" => FighterJob::Scholar,
            "Astrologian" => FighterJob::Astrologian,
            "Sage" => FighterJob::Sage,
            "Monk" => FighterJob::Monk,
            "Dragoon" => FighterJob::Dragoon,
            "Ninja" => FighterJob::Ninja,
            "Samurai" => FighterJob::Samurai,
            "Reaper" => FighterJob::Reaper,
            "Bard" => FighterJob::Bard,
            "Machinist" => FighterJob::Machinist,
            "Dancer" => FighterJob::Dancer,
            "BlackMage" => FighterJob::BlackMage,
            "Summoner" => FighterJob::Summoner,
            "RedMage" => FighterJob::RedMage,
            "BlueMage" => FighterJob::BlueMage,
            _ => unreachable!()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[cfg_attr(feature = "backend", derive(DeriveEntityModel), sea_orm(table_name = "fighter", schema_name = "final_fantasy"))]
pub struct Model {
    #[cfg_attr(feature = "backend", sea_orm(primary_key))]
    pub id: i32,
    pub job: FighterJob,
    pub level: Option<String>,
    pub gear_score: Option<String>,
    pub character_id: i32,
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
    Character
}

#[cfg(feature = "backend")]
impl Related<super::character::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Character.def()
    }
}

#[cfg(feature = "backend")]
impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn new(character_id: i32, job: FighterJob, level: String, gear_score: String) -> Self {
        Self {
            id: i32::default(),
            gear_score: Some(gear_score),
            level: Some(level),
            job,
            character_id,
        }
    }
}

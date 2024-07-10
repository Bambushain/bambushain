use std::cmp::Ordering;

#[cfg(feature = "backend")]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "backend")]
use bamboo_common_backend_macros::*;
#[cfg(feature = "frontend")]
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, EnumIter, Debug, Eq, PartialEq, Clone, Default, Copy)]
#[cfg_attr(
    feature = "backend",
    derive(DeriveActiveEnum),
    sea_orm(
        rs_type = "String",
        db_type = "Enum",
        enum_name = "final_fantasy.fighter_job"
    )
)]
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
    #[cfg_attr(feature = "backend", sea_orm(string_value = "viper"))]
    Viper,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "pictomancer"))]
    Pictomancer,
}

impl FighterJob {
    pub fn get_file_name(self) -> String {
        match self {
            FighterJob::Paladin => "paladin.webp",
            FighterJob::Warrior => "warrior.webp",
            FighterJob::DarkKnight => "darkknight.webp",
            FighterJob::Gunbreaker => "gunbreaker.webp",
            FighterJob::WhiteMage => "whitemage.webp",
            FighterJob::Scholar => "scholar.webp",
            FighterJob::Astrologian => "astrologian.webp",
            FighterJob::Sage => "sage.webp",
            FighterJob::Monk => "monk.webp",
            FighterJob::Dragoon => "dragoon.webp",
            FighterJob::Ninja => "ninja.webp",
            FighterJob::Samurai => "samurai.webp",
            FighterJob::Reaper => "reaper.webp",
            FighterJob::Bard => "bard.webp",
            FighterJob::Machinist => "machinist.webp",
            FighterJob::Dancer => "dancer.webp",
            FighterJob::BlackMage => "blackmage.webp",
            FighterJob::Summoner => "summoner.webp",
            FighterJob::RedMage => "redmage.webp",
            FighterJob::BlueMage => "bluemage.webp",
            FighterJob::Viper => "viper.webp",
            FighterJob::Pictomancer => "pictomancer.webp",
        }
        .to_string()
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
            FighterJob::Viper => "Viper",
            FighterJob::Pictomancer => "Pictomancer",
        }
        .to_string()
    }
}

impl PartialOrd for FighterJob {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FighterJob {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_job_name().cmp(&other.get_job_name())
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
            FighterJob::Viper => "Viper",
            FighterJob::Pictomancer => "Piktomant",
        }
        .to_string()
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
            "Viper" => FighterJob::Viper,
            "Pictomancer" => FighterJob::Pictomancer,
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "backend",
    derive(DeriveEntityModel, Responder),
    sea_orm(table_name = "fighter", schema_name = "final_fantasy")
)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[cfg_attr(feature = "backend", sea_orm(primary_key))]
    #[serde(default)]
    pub id: i32,
    pub job: FighterJob,
    #[serde(default)]
    pub level: Option<String>,
    #[serde(default)]
    pub gear_score: Option<String>,
    pub character_id: i32,
}

impl PartialOrd for Model {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Model {
    fn cmp(&self, other: &Self) -> Ordering {
        self.job.cmp(&other.job)
    }
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

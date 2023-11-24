use std::cmp::Ordering;

#[cfg(feature = "backend")]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(not(feature = "backend"))]
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, EnumIter, Debug, Eq, PartialEq, Clone, Default, Copy)]
#[cfg_attr(
    feature = "backend",
    derive(DeriveActiveEnum),
    sea_orm(
        rs_type = "String",
        db_type = "Enum",
        enum_name = "final_fantasy.crafter_job"
    )
)]
pub enum CrafterJob {
    #[default]
    #[cfg_attr(feature = "backend", sea_orm(string_value = "carpenter"))]
    Carpenter,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "blacksmith"))]
    Blacksmith,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "armorer"))]
    Armorer,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "goldsmith"))]
    Goldsmith,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "leatherworker"))]
    Leatherworker,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "weaver"))]
    Weaver,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "alchemist"))]
    Alchemist,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "culinarian"))]
    Culinarian,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "miner"))]
    Miner,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "botanist"))]
    Botanist,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "fisher"))]
    Fisher,
}

impl CrafterJob {
    pub fn get_file_name(self) -> String {
        match self {
            CrafterJob::Carpenter => "carpenter.webp",
            CrafterJob::Blacksmith => "blacksmith.webp",
            CrafterJob::Armorer => "armorer.webp",
            CrafterJob::Goldsmith => "goldsmith.webp",
            CrafterJob::Leatherworker => "leatherworker.webp",
            CrafterJob::Weaver => "weaver.webp",
            CrafterJob::Alchemist => "alchemist.webp",
            CrafterJob::Culinarian => "culinarian.webp",
            CrafterJob::Miner => "miner.webp",
            CrafterJob::Botanist => "botanist.webp",
            CrafterJob::Fisher => "fisher.webp",
        }
        .to_string()
    }

    pub fn get_job_name(self) -> String {
        match self {
            CrafterJob::Carpenter => "carpenter",
            CrafterJob::Blacksmith => "blacksmith",
            CrafterJob::Armorer => "armorer",
            CrafterJob::Goldsmith => "goldsmith",
            CrafterJob::Leatherworker => "leatherworker",
            CrafterJob::Weaver => "weaver",
            CrafterJob::Alchemist => "alchemist",
            CrafterJob::Culinarian => "culinarian",
            CrafterJob::Miner => "miner",
            CrafterJob::Botanist => "botanist",
            CrafterJob::Fisher => "fisher",
        }
        .to_string()
    }
}

impl PartialOrd for CrafterJob {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CrafterJob {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_job_name().cmp(&other.get_job_name())
    }
}

impl ToString for CrafterJob {
    fn to_string(&self) -> String {
        match self {
            CrafterJob::Carpenter => "Zimmerer",
            CrafterJob::Blacksmith => "Grobschmied",
            CrafterJob::Armorer => "Plattner",
            CrafterJob::Goldsmith => "Goldschmied",
            CrafterJob::Leatherworker => "Gerber",
            CrafterJob::Weaver => "Weber",
            CrafterJob::Alchemist => "Alchemist",
            CrafterJob::Culinarian => "Gourmet",
            CrafterJob::Miner => "Minenarbeiter",
            CrafterJob::Botanist => "Gärtner",
            CrafterJob::Fisher => "Fischer",
        }
        .to_string()
    }
}

impl From<String> for CrafterJob {
    fn from(value: String) -> Self {
        match value.as_str() {
            "carpenter" => CrafterJob::Carpenter,
            "blacksmith" => CrafterJob::Blacksmith,
            "armorer" => CrafterJob::Armorer,
            "goldsmith" => CrafterJob::Goldsmith,
            "leatherworker" => CrafterJob::Leatherworker,
            "weaver" => CrafterJob::Weaver,
            "alchemist" => CrafterJob::Alchemist,
            "culinarian" => CrafterJob::Culinarian,
            "miner" => CrafterJob::Miner,
            "botanist" => CrafterJob::Botanist,
            "fisher" => CrafterJob::Fisher,
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "backend",
    derive(DeriveEntityModel),
    sea_orm(table_name = "crafter", schema_name = "final_fantasy")
)]
pub struct Model {
    #[cfg_attr(feature = "backend", sea_orm(primary_key))]
    pub id: i32,
    pub job: CrafterJob,
    pub level: Option<String>,
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
    pub fn new(character_id: i32, job: CrafterJob, level: String) -> Self {
        Self {
            id: i32::default(),
            job,
            level: Some(level),
            character_id,
        }
    }
}

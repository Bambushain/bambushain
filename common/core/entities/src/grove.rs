#[cfg(not(target_arch = "wasm32"))]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
use bamboo_common_core_macros::*;

fn set_false() -> bool {
    false
}

fn set_true() -> bool {
    true
}

#[derive(Serialize, Deserialize, Debug, Eq, Ord, PartialOrd, PartialEq, Clone, Default)]
#[cfg_attr(
not(target_arch = "wasm32"),
derive(DeriveEntityModel, Responder),
sea_orm(table_name = "grove", schema_name = "grove")
)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[cfg_attr(not(target_arch = "wasm32"), sea_orm(primary_key))]
    #[serde(default)]
    pub id: i32,
    pub name: String,
    #[serde(default = "set_false")]
    pub is_suspended: bool,
    #[serde(default = "set_true")]
    pub is_enabled: bool,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user::Entity")]
    User,
    #[sea_orm(has_many = "super::event::Entity")]
    Event,
}

#[cfg(not(target_arch = "wasm32"))]
impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Related<super::event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Event.def()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn new(name: String, is_suspended: bool, is_enabled: bool) -> Self {
        Self {
            id: i32::default(),
            name,
            is_suspended,
            is_enabled,
        }
    }
}

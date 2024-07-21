#[cfg(feature = "backend")]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "backend")]
use bamboo_common_backend_macros::*;

#[derive(Serialize, Deserialize, Debug, Eq, Ord, PartialOrd, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "backend",
    derive(DeriveEntityModel, Responder),
    sea_orm(table_name = "grove", schema_name = "grove")
)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[cfg_attr(feature = "backend", sea_orm(primary_key))]
    #[serde(default)]
    pub id: i32,
    pub name: String,
}

#[cfg(feature = "backend")]
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::grove_user::Entity")]
    GroveUser,
    #[sea_orm(has_many = "super::event::Entity")]
    Event,
}

#[cfg(feature = "backend")]
impl Related<super::grove_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GroveUser.def()
    }
}

#[cfg(feature = "backend")]
impl Related<super::event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Event.def()
    }
}

#[cfg(feature = "backend")]
impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn new(name: String) -> Self {
        Self {
            id: i32::default(),
            name,
        }
    }
}

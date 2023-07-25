#[cfg(feature = "backend")]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[cfg_attr(feature = "backend", derive(DeriveEntityModel), sea_orm(table_name = "mount"))]
pub struct Model {
    #[cfg_attr(feature = "backend", sea_orm(primary_key))]
    #[serde(skip)]
    pub id: i32,
    #[cfg_attr(feature = "backend", sea_orm(unique))]
    pub name: String,
}

#[cfg(feature = "backend")]
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[cfg(feature = "backend")]
impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        super::mount_to_user::Relation::User.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::mount_to_user::Relation::Mount.def().rev())
    }
}

#[cfg(feature = "backend")]
impl ActiveModelBehavior for ActiveModel {}

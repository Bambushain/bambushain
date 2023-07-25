use chrono::NaiveDate;
#[cfg(feature = "backend")]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::user::WebUser;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[cfg_attr(feature = "backend", derive(DeriveEntityModel), sea_orm(table_name = "event"))]
pub struct Model {
    #[cfg_attr(feature = "backend", sea_orm(primary_key))]
    #[serde(skip)]
    #[cfg(feature = "backend")]
    pub id: i32,
    #[serde(skip)]
    #[cfg(feature = "backend")]
    pub user_id: i32,
    #[cfg_attr(feature = "backend", sea_orm(ignore))]
    pub username: String,
    pub time: String,
    #[serde(skip)]
    pub date: NaiveDate,
    pub available: bool,
    #[cfg_attr(feature = "backend", sea_orm(ignore))]
    pub user: WebUser,
}

#[cfg(feature = "backend")]
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
    belongs_to = "super::user::Entity",
    from = "Column::UserId",
    to = "super::user::Column::Id",
    on_update = "Cascade",
    on_delete = "Cascade"
    )]
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
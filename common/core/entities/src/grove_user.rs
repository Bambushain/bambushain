use bamboo_common_backend_macros::*;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Eq,
    Ord,
    PartialOrd,
    PartialEq,
    Clone,
    Default,
    DeriveEntityModel,
    Responder,
)]
#[sea_orm(table_name = "grove_user", schema_name = "grove")]
pub struct Model {
    pub is_mod: bool,
    pub is_banned: bool,
    #[sea_orm(primary_key)]
    pub grove_id: i32,
    #[sea_orm(primary_key)]
    pub user_id: i32,
}

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
    #[sea_orm(
        belongs_to = "super::grove::Entity",
        from = "Column::GroveId",
        to = "super::grove::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Grove,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::grove::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Grove.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

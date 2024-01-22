#[cfg(not(target_arch = "wasm32"))]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
use bamboo_common_core_macros::*;

#[derive(Serialize, Deserialize, Debug, Eq, Ord, PartialOrd, PartialEq, Clone, Default)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(DeriveEntityModel, Responder),
    sea_orm(table_name = "free_company", schema_name = "final_fantasy")
)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[cfg_attr(not(target_arch = "wasm32"), sea_orm(primary_key))]
    #[serde(default)]
    pub id: i32,
    pub name: String,
    #[serde(skip)]
    #[cfg(not(target_arch = "wasm32"))]
    pub user_id: i32,
}

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(not(target_arch = "wasm32"))]
impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn new(name: String) -> Self {
        Self {
            id: i32::default(),
            name,
            #[cfg(not(target_arch = "wasm32"))]
            user_id: i32::default(),
        }
    }
}

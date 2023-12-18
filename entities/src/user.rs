use std::fmt::{Display, Formatter};

#[cfg(not(target_arch = "wasm32"))]
use sea_orm::entity::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
use bamboo_macros::*;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(DeriveEntityModel, Responder),
    sea_orm(table_name = "user", schema_name = "authentication")
)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[cfg_attr(not(target_arch = "wasm32"), sea_orm(primary_key))]
    pub id: i32,
    #[cfg_attr(not(target_arch = "wasm32"), sea_orm(unique))]
    pub email: String,
    pub password: String,
    pub display_name: String,
    pub is_mod: bool,
    pub discord_name: String,
    #[cfg(not(target_arch = "wasm32"))]
    pub two_factor_code: Option<String>,
    #[cfg(not(target_arch = "wasm32"))]
    pub totp_secret: Option<Vec<u8>>,
    #[cfg(not(target_arch = "wasm32"))]
    #[serde(default)]
    pub totp_secret_encrypted: bool,
    pub totp_validated: Option<bool>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::character::Entity")]
    Character,
    #[sea_orm(has_many = "super::token::Entity")]
    Token,
    #[sea_orm(has_many = "super::event::Entity")]
    Event,
}

#[cfg(not(target_arch = "wasm32"))]
impl Related<super::character::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Character.def()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Related<super::token::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Token.def()
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

#[cfg(not(target_arch = "wasm32"))]
impl ActiveModel {
    pub fn set_password(&mut self, plain_password: &String) -> Result<(), bcrypt::BcryptError> {
        let hashed = bcrypt::hash(plain_password.as_bytes(), 12);
        match hashed {
            Ok(hashed_password) => {
                self.password = Set(hashed_password);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
}

impl Model {
    pub fn new(
        email: String,
        password: String,
        display_name: String,
        discord_name: String,
        is_mod: bool,
    ) -> Self {
        Self {
            id: i32::default(),
            email,
            password,
            is_mod,
            display_name,
            discord_name,
            #[cfg(not(target_arch = "wasm32"))]
            two_factor_code: None,
            #[cfg(not(target_arch = "wasm32"))]
            totp_secret: None,
            #[cfg(not(target_arch = "wasm32"))]
            totp_secret_encrypted: false,
            totp_validated: None,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn validate_password(&self, password: String) -> bool {
        let result = bcrypt::verify(password, self.password.as_str());
        match result {
            Ok(res) => res,
            Err(err) => {
                log::error!("Failed to validate password {err}");
                false
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(not(target_arch = "wasm32"), derive(Responder))]
pub struct WebUser {
    pub id: i32,
    pub display_name: String,
    pub email: String,
    pub is_mod: bool,
    pub discord_name: String,
    pub app_totp_enabled: bool,
}

impl From<Model> for WebUser {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            is_mod: value.is_mod,
            display_name: value.display_name.to_string(),
            email: value.email.to_string(),
            discord_name: value.discord_name.clone(),
            app_totp_enabled: value.totp_validated.unwrap_or(false),
        }
    }
}

impl Display for WebUser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            serde_json::to_string(self)
                .unwrap_or(self.email.clone())
                .as_str(),
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProfile {
    pub email: String,
    pub display_name: String,
    pub discord_name: String,
}

impl UpdateProfile {
    pub fn new(email: String, display_name: String, discord_name: String) -> Self {
        Self {
            email,
            display_name,
            discord_name,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ValidateTotp {
    pub code: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(not(target_arch = "wasm32"), derive(Responder))]
pub struct TotpQrCode {
    pub qr_code: String,
    pub secret: String,
}

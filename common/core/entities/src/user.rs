use std::fmt::{Display, Formatter};

#[cfg(feature = "backend")]
use sea_orm::entity::prelude::*;
#[cfg(feature = "backend")]
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

#[cfg(feature = "backend")]
use bamboo_common_backend_macros::*;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[cfg_attr(
    feature = "backend",
    derive(DeriveEntityModel, Responder),
    sea_orm(table_name = "user", schema_name = "authentication")
)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[cfg_attr(feature = "backend", sea_orm(primary_key))]
    #[serde(default)]
    pub id: i32,
    #[cfg_attr(feature = "backend", sea_orm(unique))]
    pub email: String,
    #[cfg(feature = "backend")]
    #[serde(skip)]
    pub password: String,
    pub display_name: String,
    pub is_mod: bool,
    pub discord_name: String,
    #[cfg(feature = "backend")]
    pub totp_secret: Option<Vec<u8>>,
    #[cfg(feature = "backend")]
    #[serde(default)]
    pub totp_secret_encrypted: bool,
    pub totp_validated: Option<bool>,
}

#[cfg(feature = "backend")]
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::character::Entity")]
    Character,
    #[sea_orm(has_many = "super::token::Entity")]
    Token,
    #[sea_orm(has_many = "super::event::Entity")]
    Event,
    #[sea_orm(has_many = "super::grove_user::Entity")]
    GroveUser,
}

#[cfg(feature = "backend")]
impl Related<super::character::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Character.def()
    }
}

#[cfg(feature = "backend")]
impl Related<super::token::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Token.def()
    }
}

#[cfg(feature = "backend")]
impl Related<super::event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Event.def()
    }
}

#[cfg(feature = "backend")]
impl Related<super::grove_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GroveUser.def()
    }
}

#[cfg(feature = "backend")]
impl ActiveModelBehavior for ActiveModel {}

#[cfg(feature = "backend")]
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
    pub fn new(email: String, display_name: String, discord_name: String, is_mod: bool) -> Self {
        Self {
            id: i32::default(),
            email,
            #[cfg(feature = "backend")]
            password: String::default(),
            is_mod,
            display_name,
            discord_name,
            #[cfg(feature = "backend")]
            totp_secret: None,
            #[cfg(feature = "backend")]
            totp_secret_encrypted: false,
            totp_validated: None,
        }
    }

    #[cfg(feature = "backend")]
    pub fn validate_password(&self, password: String) -> bool {
        let result = bcrypt::verify(password, self.password.as_str());
        result.unwrap_or_else(|err| {
            log::error!("Failed to validate password {err}");
            false
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "backend", derive(Responder))]
pub struct WebUser {
    #[serde(default)]
    pub id: i32,
    pub display_name: String,
    pub email: String,
    pub is_mod: bool,
    pub discord_name: String,
    #[serde(default)]
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
#[cfg_attr(feature = "backend", derive(Responder))]
pub struct TotpQrCode {
    pub qr_code: String,
    pub secret: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "backend", derive(Responder))]
pub struct GroveUser {
    #[serde(default)]
    pub id: i32,
    pub display_name: String,
    pub email: String,
    pub is_mod: bool,
}

impl From<Model> for crate::GroveUser {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            is_mod: value.is_mod,
            display_name: value.display_name.to_string(),
            email: value.email.to_string(),
        }
    }
}

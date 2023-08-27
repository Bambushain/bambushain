use std::fmt::{Display, Formatter};

#[cfg(feature = "backend")]
use sea_orm::ActiveValue::Set;
#[cfg(feature = "backend")]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[cfg_attr(feature = "backend", derive(DeriveEntityModel), sea_orm(table_name = "user", schema_name = "authentication"))]
pub struct Model {
    #[cfg_attr(feature = "backend", sea_orm(primary_key))]
    pub id: i32,
    #[cfg_attr(feature = "backend", sea_orm(unique))]
    pub email: String,
    pub password: String,
    pub display_name: String,
    pub is_mod: bool,
    pub discord_name: String,
    pub two_factor_code: Option<String>,
    #[cfg(feature = "backend")]
    pub totp_secret: Option<Vec<u8>>,
    pub totp_validated: Option<bool>,
}

#[cfg(feature = "backend")]
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::crafter::Entity")]
    Crafter,
    #[sea_orm(has_many = "super::fighter::Entity")]
    Fighter,
    #[sea_orm(has_many = "super::token::Entity")]
    Token,
}

#[cfg(feature = "backend")]
impl Related<super::crafter::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Crafter.def()
    }
}

#[cfg(feature = "backend")]
impl Related<super::fighter::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Fighter.def()
    }
}

#[cfg(feature = "backend")]
impl Related<super::token::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Token.def()
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
            Err(err) => Err(err)
        }
    }
}

impl Model {
    pub fn new(email: String, password: String, display_name: String, discord_name: String, is_mod: bool) -> Self {
        Self {
            id: i32::default(),
            email,
            password,
            is_mod,
            display_name,
            discord_name,
            two_factor_code: None,
            #[cfg(feature = "backend")]
            totp_secret: None,
            totp_validated: None,
        }
    }

    #[cfg(feature = "backend")]
    pub fn validate_password(&self, password: String) -> bool {
        bcrypt::verify(password, self.password.as_str()).unwrap_or(false)
    }

    #[cfg(feature = "backend")]
    pub fn check_totp(&self, code: String) -> bool {
        match totp_rs
        ::TOTP
        ::from_rfc6238(
            totp_rs
            ::Rfc6238
            ::new(6,
                  self.totp_secret.clone().unwrap(),
                  Some("Pandaparty".to_string()),
                  self.display_name.clone())
                .expect("Should be valid")) {
            Ok(totp) => {
                totp.check_current(code.as_str()).unwrap_or_else(|err| {
                    log::error!("Failed to validate totp {err}");
                    false
                })
            }
            Err(err) => {
                log::error!("Failed to create totp url {err}");
                false
            }
        }
    }

    pub fn to_web_user(&self) -> WebUser {
        WebUser {
            id: self.id,
            is_mod: self.is_mod,
            display_name: self.display_name.to_string(),
            email: self.email.to_string(),
            discord_name: self.discord_name.clone(),
            app_totp_enabled: self.totp_validated.unwrap_or(false),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebUser {
    pub id: i32,
    pub display_name: String,
    pub email: String,
    pub is_mod: bool,
    pub discord_name: String,
    pub app_totp_enabled: bool,
}

impl Display for WebUser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(serde_json::to_string(self).unwrap_or(self.email.clone()).as_str())
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
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TotpQrCode {
    pub qr_code: String,
    pub secret: String,
}

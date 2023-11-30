use std::fmt::{Display, Formatter};

#[cfg(feature = "backend")]
use sea_orm::entity::prelude::*;
#[cfg(feature = "backend")]
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[cfg_attr(
    feature = "backend",
    derive(DeriveEntityModel),
    sea_orm(table_name = "user", schema_name = "authentication")
)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[cfg_attr(feature = "backend", sea_orm(primary_key))]
    pub id: i32,
    #[cfg_attr(feature = "backend", sea_orm(unique))]
    pub email: String,
    pub password: String,
    pub display_name: String,
    pub is_mod: bool,
    pub discord_name: String,
    #[cfg(feature = "backend")]
    pub two_factor_code: Option<String>,
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
            two_factor_code: None,
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
        match result {
            Ok(res) => res,
            Err(err) => {
                log::error!("Failed to validate password {err}");
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
pub struct TotpQrCode {
    pub qr_code: String,
    pub secret: String,
}

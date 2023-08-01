use std::error::Error;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SheefErrorCode {
    NotFoundError,
    ExistsAlreadyError,
    InvalidDataError,
    IoError,
    DbError,
    SerializationError,
    ValidationError,
    InsufficientRightsError,
    UnauthorizedError,
    UnknownError,
}

impl Default for SheefErrorCode {
    fn default() -> Self {
        Self::UnknownError
    }
}

impl Display for SheefErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).as_str())
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SheefError {
    pub entity_type: String,
    pub error_type: SheefErrorCode,
    pub message: String,
}

impl Display for SheefError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).as_str())
    }
}

impl Error for SheefError {}

pub enum PasswordError {
    WrongPassword,
    UserNotFound,
    UnknownError,
}

#[macro_export]
macro_rules! pandaparty_not_found_error {
    ($entity_type:expr, $message:expr) => {
        pandaparty_entities::prelude::SheefError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: pandaparty_entities::prelude::SheefErrorCode::NotFoundError,
        }
    }
}

#[macro_export]
macro_rules! pandaparty_exists_already_error {
    ($entity_type:expr, $message:expr) => {
        pandaparty_entities::prelude::SheefError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: pandaparty_entities::prelude::SheefErrorCode::ExistsAlreadyError,
        }
    }
}

#[macro_export]
macro_rules! pandaparty_invalid_data_error {
    ($entity_type:expr, $message:expr) => {
        pandaparty_entities::prelude::SheefError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: pandaparty_entities::prelude::SheefErrorCode::InvalidDataError,
        }
    }
}

#[macro_export]
macro_rules! pandaparty_db_error {
    ($entity_type:expr, $message:expr) => {
        pandaparty_entities::prelude::SheefError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: pandaparty_entities::prelude::SheefErrorCode::DbError,
        }
    }
}

#[macro_export]
macro_rules! pandaparty_serialization_error {
    ($entity_type:expr, $message:expr) => {
        pandaparty_entities::prelude::SheefError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: pandaparty_entities::prelude::SheefErrorCode::SerializationError,
        }
    }
}

#[macro_export]
macro_rules! pandaparty_validation_error {
    ($entity_type:expr, $message:expr) => {
        pandaparty_entities::prelude::SheefError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: pandaparty_entities::prelude::SheefErrorCode::ValidationError,
        }
    }
}

#[macro_export]
macro_rules! pandaparty_unknown_error {
    ($entity_type:expr, $message:expr) => {
        pandaparty_entities::prelude::SheefError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: pandaparty_entities::prelude::SheefErrorCode::UnknownError,
        }
    }
}

#[macro_export]
macro_rules! pandaparty_insufficient_rights_error {
    ($entity_type:expr, $message:expr) => {
        pandaparty_entities::prelude::SheefError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: pandaparty_entities::prelude::SheefErrorCode::InsufficientRightsError,
        }
    }
}

#[macro_export]
macro_rules! pandaparty_unauthorized_error {
    ($entity_type:expr, $message:expr) => {
        pandaparty_entities::prelude::SheefError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: pandaparty_entities::prelude::SheefErrorCode::UnauthorizedError,
        }
    }
}

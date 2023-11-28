use std::error::Error;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub enum BambooErrorCode {
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

impl Default for BambooErrorCode {
    fn default() -> Self {
        Self::UnknownError
    }
}

impl Display for BambooErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).as_str())
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BambooError {
    pub entity_type: String,
    pub error_type: BambooErrorCode,
    pub message: String,
}

impl Display for BambooError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).as_str())
    }
}

impl Error for BambooError {}

pub enum PasswordError {
    WrongPassword,
    UserNotFound,
    UnknownError,
}

#[macro_export]
macro_rules! bamboo_not_found_error {
    ($entity_type:expr, $message:expr) => {
        bamboo_entities::prelude::BambooError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: bamboo_entities::prelude::BambooErrorCode::NotFoundError,
        }
    };
}

#[macro_export]
macro_rules! bamboo_exists_already_error {
    ($entity_type:expr, $message:expr) => {
        bamboo_entities::prelude::BambooError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: bamboo_entities::prelude::BambooErrorCode::ExistsAlreadyError,
        }
    };
}

#[macro_export]
macro_rules! bamboo_invalid_data_error {
    ($entity_type:expr, $message:expr) => {
        bamboo_entities::prelude::BambooError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: bamboo_entities::prelude::BambooErrorCode::InvalidDataError,
        }
    };
}

#[macro_export]
macro_rules! bamboo_db_error {
    ($entity_type:expr, $message:expr) => {
        bamboo_entities::prelude::BambooError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: bamboo_entities::prelude::BambooErrorCode::DbError,
        }
    };
}

#[macro_export]
macro_rules! bamboo_serialization_error {
    ($entity_type:expr, $message:expr) => {
        bamboo_entities::prelude::BambooError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: bamboo_entities::prelude::BambooErrorCode::SerializationError,
        }
    };
}

#[macro_export]
macro_rules! bamboo_validation_error {
    ($entity_type:expr, $message:expr) => {
        bamboo_entities::prelude::BambooError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: bamboo_entities::prelude::BambooErrorCode::ValidationError,
        }
    };
}

#[macro_export]
macro_rules! bamboo_unknown_error {
    ($entity_type:expr, $message:expr) => {
        bamboo_entities::prelude::BambooError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: bamboo_entities::prelude::BambooErrorCode::UnknownError,
        }
    };
}

#[macro_export]
macro_rules! bamboo_insufficient_rights_error {
    ($entity_type:expr, $message:expr) => {
        bamboo_entities::prelude::BambooError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: bamboo_entities::prelude::BambooErrorCode::InsufficientRightsError,
        }
    };
}

#[macro_export]
macro_rules! bamboo_unauthorized_error {
    ($entity_type:expr, $message:expr) => {
        bamboo_entities::prelude::BambooError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: bamboo_entities::prelude::BambooErrorCode::UnauthorizedError,
        }
    };
}

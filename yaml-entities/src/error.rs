use std::error::Error;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SheefErrorCode {
    NotFoundError,
    ExistsAlreadyError,
    InvalidDataError,
    IoError,
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

#[macro_export]
macro_rules! sheef_not_found_error {
    ($entity_type:expr, $message:expr) => {
        sheef_yaml_entities::SheefError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: sheef_yaml_entities::SheefErrorCode::NotFoundError,
        }
    }
}

#[macro_export]
macro_rules! sheef_exists_already_error {
    ($entity_type:expr, $message:expr) => {
        sheef_yaml_entities::SheefError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: sheef_yaml_entities::SheefErrorCode::ExistsAlreadyError,
        }
    }
}

#[macro_export]
macro_rules! sheef_invalid_data_error {
    ($entity_type:expr, $message:expr) => {
        sheef_yaml_entities::SheefError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: sheef_yaml_entities::SheefErrorCode::InvalidDataError,
        }
    }
}

#[macro_export]
macro_rules! sheef_io_error {
    ($entity_type:expr, $message:expr) => {
        sheef_yaml_entities::SheefError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: sheef_yaml_entities::SheefErrorCode::IoError,
        }
    }
}

#[macro_export]
macro_rules! sheef_serialization_error {
    ($entity_type:expr, $message:expr) => {
        sheef_yaml_entities::SheefError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: sheef_yaml_entities::SheefErrorCode::SerializationError,
        }
    }
}

#[macro_export]
macro_rules! sheef_validation_error {
    ($entity_type:expr, $message:expr) => {
        sheef_yaml_entities::SheefError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: sheef_yaml_entities::SheefErrorCode::ValidationError,
        }
    }
}

#[macro_export]
macro_rules! sheef_unknown_error {
    ($entity_type:expr, $message:expr) => {
        sheef_yaml_entities::SheefError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: sheef_yaml_entities::SheefErrorCode::UnknownError,
        }
    }
}

#[macro_export]
macro_rules! sheef_insufficient_rights_error {
    ($entity_type:expr, $message:expr) => {
        sheef_yaml_entities::SheefError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: sheef_yaml_entities::SheefErrorCode::InsufficientRightsError,
        }
    }
}

#[macro_export]
macro_rules! sheef_unauthorized_error {
    ($entity_type:expr, $message:expr) => {
        sheef_yaml_entities::SheefError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: sheef_yaml_entities::SheefErrorCode::UnauthorizedError,
        }
    }
}

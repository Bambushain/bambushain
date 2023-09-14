use std::error::Error;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PandaPartyErrorCode {
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

impl Default for PandaPartyErrorCode {
    fn default() -> Self {
        Self::UnknownError
    }
}

impl Display for PandaPartyErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).as_str())
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PandaPartyError {
    pub entity_type: String,
    pub error_type: PandaPartyErrorCode,
    pub message: String,
}

impl Display for PandaPartyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).as_str())
    }
}

impl Error for PandaPartyError {}

pub enum PasswordError {
    WrongPassword,
    UserNotFound,
    UnknownError,
}

#[macro_export]
macro_rules! pandaparty_not_found_error {
    ($entity_type:expr, $message:expr) => {
        pandaparty_entities::prelude::PandaPartyError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: pandaparty_entities::prelude::PandaPartyErrorCode::NotFoundError,
        }
    };
}

#[macro_export]
macro_rules! pandaparty_exists_already_error {
    ($entity_type:expr, $message:expr) => {
        pandaparty_entities::prelude::PandaPartyError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: pandaparty_entities::prelude::PandaPartyErrorCode::ExistsAlreadyError,
        }
    };
}

#[macro_export]
macro_rules! pandaparty_invalid_data_error {
    ($entity_type:expr, $message:expr) => {
        pandaparty_entities::prelude::PandaPartyError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: pandaparty_entities::prelude::PandaPartyErrorCode::InvalidDataError,
        }
    };
}

#[macro_export]
macro_rules! pandaparty_db_error {
    ($entity_type:expr, $message:expr) => {
        pandaparty_entities::prelude::PandaPartyError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: pandaparty_entities::prelude::PandaPartyErrorCode::DbError,
        }
    };
}

#[macro_export]
macro_rules! pandaparty_serialization_error {
    ($entity_type:expr, $message:expr) => {
        pandaparty_entities::prelude::PandaPartyError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: pandaparty_entities::prelude::PandaPartyErrorCode::SerializationError,
        }
    };
}

#[macro_export]
macro_rules! pandaparty_validation_error {
    ($entity_type:expr, $message:expr) => {
        pandaparty_entities::prelude::PandaPartyError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: pandaparty_entities::prelude::PandaPartyErrorCode::ValidationError,
        }
    };
}

#[macro_export]
macro_rules! pandaparty_unknown_error {
    ($entity_type:expr, $message:expr) => {
        pandaparty_entities::prelude::PandaPartyError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: pandaparty_entities::prelude::PandaPartyErrorCode::UnknownError,
        }
    };
}

#[macro_export]
macro_rules! pandaparty_insufficient_rights_error {
    ($entity_type:expr, $message:expr) => {
        pandaparty_entities::prelude::PandaPartyError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: pandaparty_entities::prelude::PandaPartyErrorCode::InsufficientRightsError,
        }
    };
}

#[macro_export]
macro_rules! pandaparty_unauthorized_error {
    ($entity_type:expr, $message:expr) => {
        pandaparty_entities::prelude::PandaPartyError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: pandaparty_entities::prelude::PandaPartyErrorCode::UnauthorizedError,
        }
    };
}

use std::error::Error;
use std::fmt::{Display, Formatter};

#[cfg(not(target_arch = "wasm32"))]
use actix_web::{body, http, HttpRequest, HttpResponse, Responder, ResponseError};
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
    CryptoError,
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

#[cfg(not(target_arch = "wasm32"))]
impl ResponseError for BambooError {}

#[cfg(not(target_arch = "wasm32"))]
impl Responder for BambooError {
    type Body = body::BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        match self.error_type {
            BambooErrorCode::NotFoundError => HttpResponse::NotFound(),
            BambooErrorCode::ExistsAlreadyError => HttpResponse::Conflict(),
            BambooErrorCode::UnauthorizedError => HttpResponse::Unauthorized(),
            BambooErrorCode::InsufficientRightsError => HttpResponse::Forbidden(),
            BambooErrorCode::InvalidDataError
            | BambooErrorCode::SerializationError
            | BambooErrorCode::ValidationError => HttpResponse::BadRequest(),
            BambooErrorCode::IoError
            | BambooErrorCode::DbError
            | BambooErrorCode::CryptoError
            | BambooErrorCode::UnknownError => HttpResponse::InternalServerError(),
        }
        .body(serde_json::to_string(&self).unwrap())
    }
}

pub enum PasswordError {
    WrongPassword,
    UserNotFound,
    UnknownError,
}

pub type BambooErrorResult = Result<(), BambooError>;

pub type BambooResult<T> = Result<T, BambooError>;

#[cfg(not(target_arch = "wasm32"))]
pub type BambooApiResponseResult = Result<HttpResponse, BambooError>;

#[cfg(not(target_arch = "wasm32"))]
pub type BambooApiResult<T> = Result<(T, http::StatusCode), BambooError>;

#[macro_export]
macro_rules! bamboo_not_found_error {
    ($entity_type:expr, $message:expr) => {
        bamboo_error::BambooError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: bamboo_error::BambooErrorCode::NotFoundError,
        }
    };
}

#[macro_export]
macro_rules! bamboo_exists_already_error {
    ($entity_type:expr, $message:expr) => {
        bamboo_error::BambooError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: bamboo_error::BambooErrorCode::ExistsAlreadyError,
        }
    };
}

#[macro_export]
macro_rules! bamboo_invalid_data_error {
    ($entity_type:expr, $message:expr) => {
        bamboo_error::BambooError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: bamboo_error::BambooErrorCode::InvalidDataError,
        }
    };
}

#[macro_export]
macro_rules! bamboo_db_error {
    ($entity_type:expr, $message:expr) => {
        bamboo_error::BambooError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: bamboo_error::BambooErrorCode::DbError,
        }
    };
}

#[macro_export]
macro_rules! bamboo_serialization_error {
    ($entity_type:expr, $message:expr) => {
        bamboo_error::BambooError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: bamboo_error::BambooErrorCode::SerializationError,
        }
    };
}

#[macro_export]
macro_rules! bamboo_validation_error {
    ($entity_type:expr, $message:expr) => {
        bamboo_error::BambooError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: bamboo_error::BambooErrorCode::ValidationError,
        }
    };
}

#[macro_export]
macro_rules! bamboo_unknown_error {
    ($entity_type:expr, $message:expr) => {
        bamboo_error::BambooError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: bamboo_error::BambooErrorCode::UnknownError,
        }
    };
}

#[macro_export]
macro_rules! bamboo_insufficient_rights_error {
    ($entity_type:expr, $message:expr) => {
        bamboo_error::BambooError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: bamboo_error::BambooErrorCode::InsufficientRightsError,
        }
    };
}

#[macro_export]
macro_rules! bamboo_unauthorized_error {
    ($entity_type:expr, $message:expr) => {
        bamboo_error::BambooError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: bamboo_error::BambooErrorCode::UnauthorizedError,
        }
    };
}

#[macro_export]
macro_rules! bamboo_crypto_error {
    ($entity_type:expr, $message:expr) => {
        bamboo_error::BambooError {
            entity_type: $entity_type.to_string(),
            message: $message.to_string(),
            error_type: bamboo_error::BambooErrorCode::CryptoError,
        }
    };
}

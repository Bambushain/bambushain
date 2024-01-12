use std::error::Error;
use std::fmt::{Display, Formatter};

#[cfg(not(target_arch = "wasm32"))]
use actix_web::{body, http, HttpRequest, HttpResponse, Responder, ResponseError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub enum BambooErrorCode {
    Crypto,
    Database,
    ExistsAlready,
    InsufficientRights,
    InvalidData,
    Io,
    Mailing,
    NotFound,
    Serialization,
    Unauthorized,
    Unknown,
    Validation,
}

impl Default for BambooErrorCode {
    fn default() -> Self {
        Self::Unknown
    }
}

#[cfg(target_arch = "wasm32")]
impl Display for BambooErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).as_str())
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Display for BambooErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(serde_json::to_string(self).unwrap().as_str())
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BambooError {
    pub entity_type: String,
    pub error_type: BambooErrorCode,
    pub message: String,
}

#[cfg(target_arch = "wasm32")]
impl Display for BambooError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).as_str())
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Display for BambooError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(serde_json::to_string(self).unwrap().as_str())
    }
}

impl Error for BambooError {}

#[cfg(not(target_arch = "wasm32"))]
impl ResponseError for BambooError {
    fn status_code(&self) -> http::StatusCode {
        match self.error_type {
            BambooErrorCode::NotFound => http::StatusCode::NOT_FOUND,
            BambooErrorCode::ExistsAlready => http::StatusCode::CONFLICT,
            BambooErrorCode::Unauthorized => http::StatusCode::UNAUTHORIZED,
            BambooErrorCode::InsufficientRights => http::StatusCode::FORBIDDEN,
            BambooErrorCode::InvalidData
            | BambooErrorCode::Serialization
            | BambooErrorCode::Validation => http::StatusCode::BAD_REQUEST,
            BambooErrorCode::Io
            | BambooErrorCode::Database
            | BambooErrorCode::Mailing
            | BambooErrorCode::Crypto
            | BambooErrorCode::Unknown => http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Responder for BambooError {
    type Body = body::BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        match self.error_type {
            BambooErrorCode::NotFound => HttpResponse::NotFound(),
            BambooErrorCode::ExistsAlready => HttpResponse::Conflict(),
            BambooErrorCode::Unauthorized => HttpResponse::Unauthorized(),
            BambooErrorCode::InsufficientRights => HttpResponse::Forbidden(),
            BambooErrorCode::InvalidData
            | BambooErrorCode::Serialization
            | BambooErrorCode::Validation => HttpResponse::BadRequest(),
            BambooErrorCode::Io
            | BambooErrorCode::Database
            | BambooErrorCode::Crypto
            | BambooErrorCode::Mailing
            | BambooErrorCode::Unknown => HttpResponse::InternalServerError(),
        }
            .body(serde_json::to_string(&self).unwrap())
    }
}

impl BambooError {
    fn new(
        entity_type: impl Into<String>,
        message: impl Into<String>,
        error_type: BambooErrorCode,
    ) -> Self {
        Self {
            entity_type: entity_type.into(),
            message: message.into(),
            error_type,
        }
    }

    pub fn crypto(entity_type: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(entity_type, message, BambooErrorCode::Crypto)
    }

    pub fn database(entity_type: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(entity_type, message, BambooErrorCode::Database)
    }

    pub fn exists_already(entity_type: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(entity_type, message, BambooErrorCode::ExistsAlready)
    }

    pub fn insufficient_rights(entity_type: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(entity_type, message, BambooErrorCode::InsufficientRights)
    }

    pub fn invalid_data(entity_type: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(entity_type, message, BambooErrorCode::InvalidData)
    }

    pub fn io(entity_type: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(entity_type, message, BambooErrorCode::Io)
    }

    pub fn mailing(message: impl Into<String>) -> Self {
        Self::new("mailing", message, BambooErrorCode::Mailing)
    }

    pub fn not_found(entity_type: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(entity_type, message, BambooErrorCode::NotFound)
    }

    pub fn serialization(entity_type: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(entity_type, message, BambooErrorCode::Serialization)
    }

    pub fn validation(entity_type: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(entity_type, message, BambooErrorCode::Validation)
    }

    pub fn unauthorized(entity_type: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(entity_type, message, BambooErrorCode::Unauthorized)
    }

    pub fn unknown(entity_type: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(entity_type, message, BambooErrorCode::Unknown)
    }
}

pub enum PasswordError {
    WrongPassword,
    UserNotFound,
    Unknown,
}

impl From<PasswordError> for BambooError {
    fn from(value: PasswordError) -> Self {
        match value {
            PasswordError::WrongPassword => {
                BambooError::insufficient_rights("user", "The current password is wrong")
            }
            PasswordError::UserNotFound => BambooError::not_found("user", "The user was not found"),
            PasswordError::Unknown => BambooError::unknown("user", "An unknown error occurred"),
        }
    }
}

pub type BambooErrorResult = Result<(), BambooError>;

pub type BambooResult<T> = Result<T, BambooError>;

#[cfg(not(target_arch = "wasm32"))]
pub type BambooApiResponseResult = Result<HttpResponse, BambooError>;

#[cfg(not(target_arch = "wasm32"))]
pub type BambooApiResult<T> = Result<(T, http::StatusCode), BambooError>;

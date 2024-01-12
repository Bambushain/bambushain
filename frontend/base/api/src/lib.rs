use std::convert::Into;
use std::fmt::{Debug, Display, Formatter};

use gloo_net::http::{Request, Response};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

use bamboo_error::*;
use bamboo_frontend_base_storage::get_token;

macro_rules! error_code {
    ($name:tt,$code:literal) => {
        pub const $name: ErrorCode = ErrorCode($code);
    };
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct ErrorCode(i32);

impl Debug for ErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", self.0).as_str())
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", self.0).as_str())
    }
}

impl std::error::Error for ErrorCode {}

impl From<i32> for ErrorCode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<u16> for ErrorCode {
    fn from(value: u16) -> Self {
        Self(value as i32)
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct ApiError {
    pub code: ErrorCode,
    pub bamboo_error: BambooError,
}

impl Default for ApiError {
    fn default() -> Self {
        Self {
            code: ErrorCode::from(0),
            bamboo_error: BambooError::default(),
        }
    }
}

impl ApiError {
    pub fn new(status: u16, error: bamboo_error::BambooError) -> Self {
        ApiError {
            code: ErrorCode::from(status),
            bamboo_error: error,
        }
    }

    pub fn send_error() -> Self {
        Self {
            code: SEND_ERROR,
            bamboo_error: bamboo_error::BambooError::default(),
        }
    }

    pub fn json_deserialize_error() -> Self {
        Self {
            code: JSON_DESERIALIZE_ERROR,
            bamboo_error: bamboo_error::BambooError::default(),
        }
    }

    pub fn json_serialize_error() -> Self {
        Self {
            code: JSON_SERIALIZE_ERROR,
            bamboo_error: bamboo_error::BambooError::default(),
        }
    }
}

impl std::error::Error for ApiError {}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).as_str())
    }
}

pub type BambooApiResult<T> = Result<T, ApiError>;

error_code!(SEND_ERROR, -1);
error_code!(JSON_SERIALIZE_ERROR, -2);
error_code!(JSON_DESERIALIZE_ERROR, -3);
error_code!(NO_CONTENT, 204);
error_code!(FORBIDDEN, 403);
error_code!(NOT_FOUND, 404);
error_code!(CONFLICT, 409);
error_code!(INTERNAL_SERVER_ERROR, 500);

macro_rules! request_with_response {
    ($func_name:ident, $method:tt) => {
        pub async fn $func_name<OUT: DeserializeOwned>(
            uri: impl Into<String>,
        ) -> BambooApiResult<OUT> {
            let uri = uri.into();
            let token = get_token().unwrap_or_default();
            log::debug!("Use auth token {token}");
            log::debug!("Execute $method request against {uri}");
            let response = Request::$method(uri.as_str())
                .header("Authorization", format!("Panda {token}").as_str())
                .send()
                .await
                .map_err(|_| ApiError::send_error())?;

            handle_response(response).await
        }
    };
}

macro_rules! request_with_response_no_content {
    ($func_name:ident, $method:tt) => {
        pub async fn $func_name<IN: Serialize>(
            uri: impl Into<String>,
            body: &IN,
        ) -> BambooApiResult<()> {
            let uri = uri.into();
            let token = get_token().unwrap_or_default();
            log::debug!("Use auth token {token}");
            log::debug!("Execute $method request against {uri}");
            let request = Request::$method(uri.as_str())
                .header("Authorization", format!("Panda {token}").as_str())
                .json(body)
                .map_err(|_| ApiError::json_serialize_error())?
                .send()
                .await
                .map_err(|_| ApiError::send_error())?;

            handle_response_code(request).await
        }
    };
}

async fn handle_response<OUT: DeserializeOwned>(response: Response) -> BambooApiResult<OUT> {
    log::debug!("Request executed successfully");
    let status = response.status();
    log::debug!("Response status code is {status}");
    if 199 < status && 300 > status {
        let text = response.text().await.unwrap();
        log::trace!("Response body: {text}");
        Ok(serde_json::from_str(text.as_str()).map_err(|_| ApiError::json_deserialize_error())?)
    } else {
        log::debug!("Request status code is not in success range (200-299)");
        let text = response.text().await.unwrap();
        log::trace!("Error text: {text}");

        Err(serde_json::from_str(text.as_str())
            .map_err(|_| ApiError::json_deserialize_error())
            .map(|err| ApiError::new(response.status(), err))?)
    }
}

async fn handle_response_code(response: Response) -> BambooApiResult<()> {
    log::debug!("Request executed successfully");
    let status = response.status();
    log::debug!("Response status code is {status}");
    if 199 < status && 300 > status {
        let text = response.text().await.unwrap();
        log::trace!("Response body: {text}");
        Ok(())
    } else {
        log::debug!("Request status code is not in success range (200-299)");
        let text = response.text().await.unwrap();
        log::trace!("Error text: {text}");

        Err(serde_json::from_str(text.as_str())
            .map_err(|_| ApiError::json_deserialize_error())
            .map(|err| ApiError::new(response.status(), err))?)
    }
}

request_with_response!(get, get);
request_with_response!(post_no_body, post);

request_with_response_no_content!(post_no_content, post);
request_with_response_no_content!(put_no_content, put);

pub async fn get_with_query<OUT: DeserializeOwned, Value: AsRef<str>>(
    uri: impl Into<String>,
    query: Vec<(&str, Value)>,
) -> BambooApiResult<OUT> {
    let uri = uri.into();
    let token = get_token().unwrap_or_default();
    log::debug!("Use auth token {token}");
    log::debug!("Execute get request against {uri}");
    let response = Request::get(uri.as_str())
        .query(query.into_iter())
        .header("Authorization", format!("Panda {token}").as_str())
        .send()
        .await
        .map_err(|_| ApiError::send_error())?;

    handle_response(response).await
}

pub async fn post<IN: Serialize, OUT: DeserializeOwned>(
    uri: impl Into<String>,
    body: &IN,
) -> BambooApiResult<OUT> {
    let uri = uri.into();
    let token = get_token().unwrap_or_default();
    log::debug!("Use auth token {token}");
    log::debug!("Execute post request against {uri}");
    let request = Request::post(uri.as_str())
        .header("Authorization", format!("Panda {token}").as_str())
        .json(body)
        .map_err(|_| ApiError::json_serialize_error())?
        .send()
        .await
        .map_err(|_| ApiError::send_error())?;

    handle_response(request).await
}

pub async fn delete(uri: impl Into<String>) -> BambooApiResult<()> {
    let uri = uri.into();
    let token = get_token().unwrap_or_default();
    log::debug!("Use auth token {token}");
    log::debug!("Execute $method request against {uri}");
    let request = Request::delete(uri.as_str())
        .header("Authorization", format!("Panda {token}").as_str())
        .send()
        .await
        .map_err(|_| ApiError::send_error())?;

    handle_response_code(request).await
}

pub async fn put_no_body_no_content(uri: impl Into<String>) -> BambooApiResult<()> {
    let uri = uri.into();
    let token = get_token().unwrap_or_default();
    log::debug!("Use auth token {token}");
    log::debug!("Execute $method request against {uri}");
    let request = Request::put(uri.as_str())
        .header("Authorization", format!("Panda {token}").as_str())
        .send()
        .await
        .map_err(|_| ApiError::send_error())?;

    handle_response_code(request).await
}

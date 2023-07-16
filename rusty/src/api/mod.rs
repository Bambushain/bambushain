use std::convert::Into;
use std::fmt::{Debug, Display, Formatter};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use crate::storage::get_token;

pub mod authentication;
pub mod my;
pub mod calendar;
pub mod user;
pub mod crafter;
pub mod fighter;

macro_rules! error_code {
    ($name:tt,$code:literal) => {
        pub const $name: ErrorCode = ErrorCode($code);
    };
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Serialize, Deserialize)]
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

error_code!(SEND_ERROR, -1);
error_code!(JSON_SERIALIZE_ERROR, -2);
error_code!(JSON_DESERIALIZE_ERROR, -3);
error_code!(NO_CONTENT, 204);
error_code!(FORBIDDEN, 403);
error_code!(NOT_FOUND, 404);
error_code!(CONFLICT, 409);
error_code!(INTERNAL_SERVER_ERROR, 500);

macro_rules! handle_response {
    ($response:expr) => {
        {
            let json_result = match $response {
                Ok(response) => {
                    log::debug!("Request executed successfully");
                    let status = response.status();
                    log::debug!("Response status code is {}", status);
                    if 199 < status && 300 > status {
                        let text = response.text().await.unwrap();
                        log::trace!("Response body: {text}");
                        serde_json::from_str(text.as_str())
                    } else {
                        log::debug!("Request status code is not in success range (200-299)");
                        return Err(ErrorCode::from(response.status() as i32));
                    }
                }
                Err(err) => {
                    log::warn!("Request failed to execute {}", err);
                    return Err(SEND_ERROR);
                }
            };

            match json_result {
                Ok(result) => {
                    log::debug!("Json deserialize was successful");
                    Ok(result)
                }
                Err(err) => {
                    log::warn!("Json deserialize failed {}", err);
                    Err(JSON_DESERIALIZE_ERROR)
                }
            }
        }
    };
}

macro_rules! handle_response_code {
    ($response:expr) => {
        {
            match $response {
                Ok(response) => {
                    log::debug!("Request executed successfully");
                    let status = response.status();
                    log::debug!("Response status code is {}", status);
                    ErrorCode(status.into())
                }
                Err(err) => {
                    log::warn!("Request failed to execute {}", err);
                    SEND_ERROR
                }
            }
        }
    };
}

pub async fn get<OUT>(uri: impl Into<String>) -> Result<OUT, ErrorCode> where OUT: DeserializeOwned {
    let into_uri = uri.into();
    let token = get_token().unwrap_or_default();
    log::debug!("Use auth token {}", token);
    log::debug!("Execute get request against {}", &into_uri);
    let response = gloo::net::http::Request::get(into_uri.as_str())
        .header("Authorization", format!("Sheef {}", token).as_str())
        .send()
        .await;

    handle_response!(response)
}

pub async fn delete(uri: impl Into<String>) -> ErrorCode {
    let into_uri = uri.into();
    let token = get_token().unwrap_or_default();
    log::debug!("Use auth token {}", token);
    log::debug!("Execute get request against {}", &into_uri);
    let response = gloo::net::http::Request::delete(into_uri.as_str())
        .header("Authorization", format!("Sheef {}", token).as_str())
        .send()
        .await;

    handle_response_code!(response)
}

pub async fn put_no_body(uri: impl Into<String>) -> ErrorCode {
    let into_uri = uri.into();
    let token = get_token().unwrap_or_default();
    log::debug!("Use auth token {}", token);
    log::debug!("Execute get request against {}", &into_uri);
    let response = gloo::net::http::Request::put(into_uri.as_str())
        .header("Authorization", format!("Sheef {}", token).as_str())
        .send()
        .await;

    handle_response_code!(response)
}

pub async fn put<IN>(uri: impl Into<String>, body: &IN) -> ErrorCode where IN: Serialize {
    let into_uri = uri.into();
    let token = get_token().unwrap_or_default();
    log::debug!("Use auth token {}", token);
    log::debug!("Execute get request against {}", &into_uri);
    match gloo::net::http::Request::put(into_uri.as_str())
        .header("Authorization", format!("Sheef {}", token).as_str())
        .json(body) {
        Ok(request) => handle_response_code!(request.send().await),
        Err(err) => {
            log::warn!("Serialize failed {}", err);
            JSON_SERIALIZE_ERROR
        }
    }
}

pub async fn post<IN, OUT>(uri: impl Into<String>, body: &IN) -> Result<OUT, ErrorCode> where IN: Serialize, OUT: DeserializeOwned {
    let into_uri = uri.into();
    let token = get_token().unwrap_or_default();
    log::debug!("Use auth token {}", token);
    let token = get_token().unwrap_or_default();

    log::debug!("Execute post request against {}", &into_uri);
    match gloo::net::http::Request::post(into_uri.as_str())
        .header("Authorization", format!("Sheef {}", token).as_str())
        .json(body) {
        Ok(request) => handle_response!(request.send().await),
        Err(err) => {
            log::warn!("Serialize failed {}", err);
            Err(JSON_SERIALIZE_ERROR)
        }
    }
}

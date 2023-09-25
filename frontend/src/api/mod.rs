use std::convert::Into;
use std::fmt::{Debug, Display, Formatter};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub use authentication::*;
pub use character::*;
pub use crafter::*;
pub use custom_field::*;
pub use event::*;
pub use fighter::*;
pub use my::*;
use pandaparty_entities::prelude::*;
pub use user::*;

use crate::storage::get_token;

pub mod authentication;
pub mod character;
pub mod crafter;
pub mod custom_field;
pub mod event;
pub mod fighter;
pub mod free_company;
pub mod my;
pub mod user;

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

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct ApiError {
    pub code: ErrorCode,
    pub pandaparty_error: PandaPartyError,
}

impl std::error::Error for ApiError {}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).as_str())
    }
}

pub type PandapartyApiResult<T> = Result<T, ApiError>;

error_code!(SEND_ERROR, -1);
error_code!(JSON_SERIALIZE_ERROR, -2);
error_code!(JSON_DESERIALIZE_ERROR, -3);
error_code!(NO_CONTENT, 204);
error_code!(FORBIDDEN, 403);
error_code!(NOT_FOUND, 404);
error_code!(CONFLICT, 409);
error_code!(INTERNAL_SERVER_ERROR, 500);

macro_rules! handle_response {
    ($response:expr) => {{
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
                    let text = response.text().await.unwrap();
                    log::trace!("Error text: {text}");
                    let error =
                        serde_json::from_str(text.as_str()).expect("Should deserialize the data");

                    return Err(crate::api::ApiError {
                        code: crate::api::ErrorCode::from(response.status() as i32),
                        pandaparty_error: error,
                    });
                }
            }
            Err(err) => {
                log::warn!("Request failed to execute {}", err);
                return Err(crate::api::ApiError {
                    code: SEND_ERROR,
                    pandaparty_error: pandaparty_entities::prelude::PandaPartyError::default(),
                });
            }
        };

        match json_result {
            Ok(result) => {
                log::debug!("Json deserialize was successful");
                Ok(result)
            }
            Err(err) => {
                log::warn!("Json deserialize failed {}", err);
                Err(crate::api::ApiError {
                    code: JSON_DESERIALIZE_ERROR,
                    pandaparty_error: pandaparty_entities::prelude::PandaPartyError::default(),
                })
            }
        }
    }};
}

macro_rules! handle_response_code {
    ($response:expr) => {{
        match $response {
            Ok(response) => {
                log::debug!("Request executed successfully");
                let status = response.status();
                log::debug!("Response status code is {}", status);
                if 199 < status && 300 > status {
                    let text = response.text().await.unwrap();
                    log::trace!("Response body: {text}");
                    Ok(())
                } else {
                    log::debug!("Request status code is not in success range (200-299)");
                    let text = response.text().await.unwrap();
                    log::trace!("Error text: {text}");
                    let error =
                        serde_json::from_str(text.as_str()).expect("Should deserialize the data");

                    return Err(crate::api::ApiError {
                        code: crate::api::ErrorCode::from(response.status() as i32),
                        pandaparty_error: error,
                    });
                }
            }
            Err(err) => {
                log::warn!("Request failed to execute {}", err);
                Err(ApiError {
                    code: SEND_ERROR,
                    pandaparty_error: pandaparty_entities::prelude::PandaPartyError::default(),
                })
            }
        }
    }};
}

pub async fn get<OUT: DeserializeOwned>(uri: impl Into<String>) -> PandapartyApiResult<OUT> {
    let into_uri = uri.into();
    let token = get_token().unwrap_or_default();
    log::debug!("Use auth token {}", token);
    log::debug!("Execute get request against {}", &into_uri);
    let response = gloo::net::http::Request::get(into_uri.as_str())
        .header("Authorization", format!("Panda {}", token).as_str())
        .send()
        .await;

    handle_response!(response)
}

pub async fn get_with_query<OUT: DeserializeOwned, Value: AsRef<str>>(
    uri: impl Into<String>,
    query: Vec<(&str, Value)>,
) -> PandapartyApiResult<OUT> {
    let into_uri = uri.into();
    let token = get_token().unwrap_or_default();
    log::debug!("Use auth token {}", token);
    log::debug!("Execute get request against {}", &into_uri);
    let response = gloo::net::http::Request::get(into_uri.as_str())
        .query(query.into_iter())
        .header("Authorization", format!("Panda {}", token).as_str())
        .send()
        .await;

    handle_response!(response)
}

pub async fn delete(uri: impl Into<String>) -> PandapartyApiResult<()> {
    let into_uri = uri.into();
    let token = get_token().unwrap_or_default();
    log::debug!("Use auth token {}", token);
    log::debug!("Execute get request against {}", &into_uri);
    let response = gloo::net::http::Request::delete(into_uri.as_str())
        .header("Authorization", format!("Panda {}", token).as_str())
        .send()
        .await;

    handle_response_code!(response)
}

pub async fn put_no_body_no_content(uri: impl Into<String>) -> PandapartyApiResult<()> {
    let into_uri = uri.into();
    let token = get_token().unwrap_or_default();
    log::debug!("Use auth token {}", token);
    log::debug!("Execute get request against {}", &into_uri);
    let response = gloo::net::http::Request::put(into_uri.as_str())
        .header("Authorization", format!("Panda {}", token).as_str())
        .send()
        .await;

    handle_response_code!(response)
}

pub async fn put_no_content<IN: Serialize>(
    uri: impl Into<String>,
    body: &IN,
) -> PandapartyApiResult<()> {
    let into_uri = uri.into();
    let token = get_token().unwrap_or_default();
    log::debug!("Use auth token {}", token);
    log::debug!("Execute get request against {}", &into_uri);
    match gloo::net::http::Request::put(into_uri.as_str())
        .header("Authorization", format!("Panda {}", token).as_str())
        .json(body)
    {
        Ok(request) => handle_response_code!(request.send().await),
        Err(err) => {
            log::warn!("Serialize failed {}", err);
            Err(ApiError {
                pandaparty_error: PandaPartyError::default(),
                code: JSON_SERIALIZE_ERROR,
            })
        }
    }
}

pub async fn post<IN: Serialize, OUT: DeserializeOwned>(
    uri: impl Into<String>,
    body: &IN,
) -> PandapartyApiResult<OUT> {
    let into_uri = uri.into();
    let token = get_token().unwrap_or_default();
    log::debug!("Use auth token {}", token);
    let token = get_token().unwrap_or_default();

    log::debug!("Execute post request against {}", &into_uri);
    match gloo::net::http::Request::post(into_uri.as_str())
        .header("Authorization", format!("Panda {}", token).as_str())
        .json(body)
    {
        Ok(request) => handle_response!(request.send().await),
        Err(err) => {
            log::warn!("Serialize failed {}", err);
            Err(ApiError {
                pandaparty_error: PandaPartyError::default(),
                code: JSON_SERIALIZE_ERROR,
            })
        }
    }
}

pub async fn post_no_content<IN: Serialize>(
    uri: impl Into<String>,
    body: &IN,
) -> PandapartyApiResult<()> {
    let into_uri = uri.into();
    let token = get_token().unwrap_or_default();
    log::debug!("Use auth token {}", token);
    let token = get_token().unwrap_or_default();

    log::debug!("Execute post request against {}", &into_uri);
    match gloo::net::http::Request::post(into_uri.as_str())
        .header("Authorization", format!("Panda {}", token).as_str())
        .json(body)
    {
        Ok(request) => handle_response_code!(request.send().await),
        Err(err) => {
            log::warn!("Serialize failed {}", err);
            Err(ApiError {
                pandaparty_error: PandaPartyError::default(),
                code: JSON_SERIALIZE_ERROR,
            })
        }
    }
}

pub async fn post_no_body<OUT: DeserializeOwned>(
    uri: impl Into<String>,
) -> PandapartyApiResult<OUT> {
    let into_uri = uri.into();
    let token = get_token().unwrap_or_default();
    log::debug!("Use auth token {}", token);
    let token = get_token().unwrap_or_default();

    log::debug!("Execute post request against {}", &into_uri);
    handle_response!(
        gloo::net::http::Request::post(into_uri.as_str())
            .header("Authorization", format!("Panda {}", token).as_str())
            .send()
            .await
    )
}

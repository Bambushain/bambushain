use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use serde::Serialize;

use bamboo_common::frontend::api::*;
use bamboo_pandas_frontend_base_storage::get_token;

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

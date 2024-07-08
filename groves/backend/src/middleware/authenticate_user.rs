use actix_web::{body, dev, Error, HttpMessage};
use actix_web_lab::middleware::Next;
use openidconnect::AccessToken;
use bamboo_common::backend::services::EnvService;
use bamboo_common::core::error::BambooError;

use crate::authentication::{get_client, validate_user};

pub type Username = String;

pub async fn authenticate_user(
    req: dev::ServiceRequest,
    next: Next<impl body::MessageBody>,
) -> Result<dev::ServiceResponse<impl body::MessageBody>, Error> {
    let env_service = req
        .app_data::<EnvService>()
        .ok_or(BambooError::unauthorized("login", "Invalid data"))?;
    let client = get_client(env_service.clone()).await?;
    let authorization_header = req.headers().get("Authorization");
    if let Some(authorization_header) = authorization_header {
        let access_token = authorization_header
            .to_str()
            .map_err(|_| BambooError::unauthorized("login", "Invalid header"))?
            .to_string();
        if let Some(access_token) = access_token.strip_prefix("Bearer ") {
            let name = validate_user(AccessToken::new(access_token.to_string()), client).await?;

            req.extensions_mut().insert(name as Username);

            next.call(req).await
        } else {
            Err(BambooError::unauthorized("login", "Invalid header").into())
        }
    } else {
        Err(BambooError::unauthorized("login", "Invalid authorization").into())
    }
}

macro_rules! authenticate {
    () => {
        actix_web_lab::middleware::from_fn(crate::middleware::authenticate_user::authenticate_user)
    };
}

pub(crate) use authenticate;

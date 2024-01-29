use actix_session::Session;
use actix_web::{body, dev, Error, HttpMessage};
use actix_web_lab::middleware::Next;
use openidconnect::AccessToken;

use bamboo_common::backend::services::EnvService;
use bamboo_common::core::error::BambooError;

use crate::authentication::{get_client, validate_user, ACCESS_TOKEN};

pub type Username = String;

pub async fn authenticate_user(
    session: Session,
    req: dev::ServiceRequest,
    next: Next<impl body::MessageBody>,
) -> Result<dev::ServiceResponse<impl body::MessageBody>, Error> {
    let connection_info = req.connection_info().clone();
    let host = connection_info.host();
    let env_service = req
        .app_data::<EnvService>()
        .ok_or(BambooError::unauthorized("login", "Invalid data"))?;
    let client = get_client(host.into(), env_service.clone()).await?;
    let access_token = session
        .get::<AccessToken>(ACCESS_TOKEN)
        .map_err(|err| {
            log::error!("{ACCESS_TOKEN}: {err}");
            BambooError::unauthorized("login", "Invalid session")
        })?
        .ok_or_else(|| BambooError::unauthorized("login", "Invalid session"))?;
    let name = validate_user(access_token, client).await?;
    session.renew();

    req.extensions_mut().insert(name as Username);

    next.call(req).await
}

macro_rules! authenticate {
    () => {
        actix_web_lab::middleware::from_fn(crate::middleware::authenticate_user::authenticate_user)
    };
}

pub(crate) use authenticate;

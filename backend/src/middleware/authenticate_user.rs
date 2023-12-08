use actix_web::{body, dev, error::ParseError, http::header, web, Error, HttpMessage};
use actix_web_lab::middleware::Next;
use serde::{Deserialize, Serialize};

use bamboo_dbal::prelude::*;
use bamboo_entities::prelude::*;
use bamboo_error::bamboo_unauthorized_error;
use bamboo_services::prelude::DbConnection;

#[derive(Clone)]
pub(crate) struct AuthenticationState {
    pub token: String,
    pub user: User,
}

pub(crate) type Authentication = web::ReqData<AuthenticationState>;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub(crate) struct AuthorizationHeader {
    pub authorization: Option<String>,
}

impl header::TryIntoHeaderValue for AuthorizationHeader {
    type Error = header::InvalidHeaderValue;

    fn try_into_value(self) -> Result<header::HeaderValue, Self::Error> {
        header::HeaderValue::from_str(self.authorization.unwrap_or_default().as_str())
    }
}

impl header::Header for AuthorizationHeader {
    fn name() -> header::HeaderName {
        header::AUTHORIZATION
    }

    fn parse<M: HttpMessage>(msg: &M) -> Result<Self, ParseError> {
        let authorization = if let Some(header) = msg.headers().get(header::AUTHORIZATION) {
            Ok(header)
        } else {
            Err(ParseError::Header)
        }?
        .to_str()
        .map_err(|_| ParseError::Header)
        .map(|header| header.strip_prefix("Panda ").map(|res| res.to_string()))?;

        Ok(AuthorizationHeader { authorization })
    }
}

pub(crate) async fn authenticate_user(
    db: DbConnection,
    authorization: Option<web::Header<AuthorizationHeader>>,
    req: dev::ServiceRequest,
    next: Next<impl body::MessageBody>,
) -> Result<dev::ServiceResponse<impl body::MessageBody>, Error> {
    let unauthorized = bamboo_unauthorized_error!("user", "Authorization failed");
    let token = if let Some(header) = authorization {
        if let Some(authorization) = header.authorization.clone() {
            Ok(authorization)
        } else {
            Err(unauthorized.clone())
        }
    } else {
        Err(unauthorized.clone())
    }?;

    let user = get_user_by_token(token.clone(), &db)
        .await
        .map_err(|_| unauthorized.clone())?;

    req.extensions_mut()
        .insert(AuthenticationState { token, user });

    next.call(req).await
}

macro_rules! authenticate {
    () => {
        actix_web_lab::middleware::from_fn(crate::middleware::authenticate_user::authenticate_user)
    };
}

pub(crate) use authenticate;

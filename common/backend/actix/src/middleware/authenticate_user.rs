use actix_web::{body, dev, web, Error, HttpMessage};
use actix_web_lab::middleware::Next;

use bamboo_common_backend_services::DbConnection;
use bamboo_common_core::entities::*;

use crate::cookie;
use crate::header;
use crate::middleware::helpers;

#[derive(Clone)]
pub struct AuthenticationState {
    pub token: String,
    pub user: User,
}

pub type Authentication = web::ReqData<AuthenticationState>;

pub async fn authenticate_user(
    db: DbConnection,
    authorization: Option<web::Header<header::AuthorizationHeader>>,
    auth_cookie: Option<cookie::BambooAuthCookie>,
    req: dev::ServiceRequest,
    next: Next<impl body::MessageBody>,
) -> Result<dev::ServiceResponse<impl body::MessageBody>, Error> {
    let (token, user) = if authorization.is_some() {
        helpers::get_user_and_token_by_header(&db, authorization).await?
    } else {
        helpers::get_user_and_token_by_cookie(&db, auth_cookie).await?
    };

    req.extensions_mut()
        .insert(AuthenticationState { token, user });

    next.call(req).await
}

#[macro_export]
macro_rules! authenticate {
    () => {
        actix_web_lab::middleware::from_fn(
            bamboo_common::backend::actix::middleware::authenticate_user,
        )
    };
}

pub use authenticate;

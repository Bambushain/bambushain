use crate::path;
use actix_web::{body, dev, web, Error};
use actix_web_lab::middleware::Next;
use bamboo_common::backend::actix::{cookie, header, middleware};
use bamboo_common::backend::dbal;
use bamboo_common::backend::services::DbConnection;
use bamboo_common::core::error::BambooError;

pub(crate) async fn check_grove_mod(
    db: DbConnection,
    path: Option<path::GrovePath>,
    authorization: Option<web::Header<header::AuthorizationHeader>>,
    auth_cookie: Option<cookie::BambooAuthCookie>,
    req: dev::ServiceRequest,
    next: Next<impl body::MessageBody>,
) -> Result<dev::ServiceResponse<impl body::MessageBody>, Error> {
    let (_, user) = if authorization.is_some() {
        middleware::get_user_and_token_by_header(&db, authorization).await?
    } else {
        middleware::get_user_and_token_by_cookie(&db, auth_cookie).await?
    };

    if let Some(path) = path {
        if !dbal::is_grove_mod(path.grove_id, user.id, &db).await? {
            return Err(BambooError::insufficient_rights(
                "grove",
                "You don't have the right to manage this grove",
            )
            .into());
        }
    }

    next.call(req).await
}

macro_rules! grove_mod {
    () => {
        actix_web_lab::middleware::from_fn(crate::middleware::check_grove_mod::check_grove_mod)
    };
}

pub(crate) use grove_mod;

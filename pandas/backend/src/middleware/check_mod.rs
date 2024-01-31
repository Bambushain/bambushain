use actix_web::{body, dev, web, Error};
use actix_web_lab::middleware::Next;

use bamboo_common::backend::services::DbConnection;
use bamboo_common::core::error::BambooError;

use crate::header;
use crate::middleware::helpers;

pub(crate) async fn check_mod(
    db: DbConnection,
    authorization: Option<web::Header<header::AuthorizationHeader>>,
    req: dev::ServiceRequest,
    next: Next<impl body::MessageBody>,
) -> Result<dev::ServiceResponse<impl body::MessageBody>, Error> {
    let (_, user) = helpers::get_user_and_token_by_header(&db, authorization).await?;

    if user.is_mod {
        next.call(req).await
    } else {
        Err(BambooError::insufficient_rights("user", "You need to be a mod").into())
    }
}

macro_rules! is_mod {
    () => {
        actix_web_lab::middleware::from_fn(crate::middleware::check_mod::check_mod)
    };
}

pub(crate) use is_mod;

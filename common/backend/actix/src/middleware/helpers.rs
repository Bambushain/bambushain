use actix_web::web;
use sea_orm::DatabaseConnection;

use bamboo_common_backend_dbal as dbal;
use bamboo_common_core::entities::*;
use bamboo_common_core::error::{BambooError, BambooResult};

use crate::cookie;
use crate::header;

pub async fn get_user_and_token_by_header(
    db: &DatabaseConnection,
    authorization: Option<web::Header<header::AuthorizationHeader>>,
) -> BambooResult<(String, User)> {
    let unauthorized = BambooError::unauthorized("user", "Authorization failed");
    let token = authorization.map_or_else(
        || Err(unauthorized.clone()),
        |header| header.authorization.clone().ok_or(unauthorized.clone()),
    )?;

    let user = dbal::get_user_by_token(token.clone(), db)
        .await
        .map_err(|_| unauthorized.clone())?;

    Ok((token, user))
}

pub async fn get_user_and_token_by_cookie(
    db: &DatabaseConnection,
    auth_cookie: Option<cookie::BambooAuthCookie>,
) -> BambooResult<(String, User)> {
    let unauthorized = BambooError::unauthorized("user", "Authorization failed");
    let token = auth_cookie.ok_or(unauthorized.clone())?.token;

    let user = dbal::get_user_by_token(token.clone(), db)
        .await
        .map_err(|_| unauthorized.clone())?;

    Ok((token, user))
}

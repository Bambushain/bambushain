use actix_web::web;
use sea_orm::DatabaseConnection;

use bamboo_dbal::prelude::dbal;
use bamboo_entities::prelude::*;
use bamboo_error::{BambooError, BambooResult};

use crate::cookie;
use crate::header;

pub async fn get_user_and_token_by_header(
    db: &DatabaseConnection,
    authorization: Option<web::Header<header::AuthorizationHeader>>,
) -> BambooResult<(String, User)> {
    let unauthorized = BambooError::unauthorized("user", "Authorization failed");
    let token = if let Some(header) = authorization {
        if let Some(authorization) = header.authorization.clone() {
            Ok(authorization)
        } else {
            Err(unauthorized.clone())
        }
    } else {
        Err(unauthorized.clone())
    }?;

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
    let token = if let Some(cookie) = auth_cookie {
        Ok(cookie.token.clone())
    } else {
        Err(unauthorized.clone())
    }?;

    let user = dbal::get_user_by_token(token.clone(), db)
        .await
        .map_err(|_| unauthorized.clone())?;

    Ok((token, user))
}

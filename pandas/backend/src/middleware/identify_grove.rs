use actix_web::{body, dev, Error, HttpMessage, web};
use actix_web_lab::middleware::Next;

use bamboo_common::backend::dbal;
use bamboo_common::backend::services::DbConnection;
use bamboo_common::core::entities::Grove as DbGrove;

use crate::cookie;
use crate::header;
use crate::middleware::helpers;

#[derive(Clone)]
pub(crate) struct GroveState {
    pub grove: DbGrove,
}

pub(crate) type CurrentGrove = web::ReqData<GroveState>;

pub(crate) async fn identify_grove(
    db: DbConnection,
    authorization: Option<web::Header<header::AuthorizationHeader>>,
    auth_cookie: Option<cookie::BambooAuthCookie>,
    req: dev::ServiceRequest,
    next: Next<impl body::MessageBody>,
) -> Result<dev::ServiceResponse<impl body::MessageBody>, Error> {
    let (_, user) = if authorization.is_some() {
        helpers::get_user_and_token_by_header(&db, authorization).await?
    } else {
        helpers::get_user_and_token_by_cookie(&db, auth_cookie).await?
    };

    let grove = dbal::get_grove_by_user_id(user.id, &db).await?;

    req.extensions_mut().insert(GroveState { grove });

    next.call(req).await
}

macro_rules! grove {
    () => {
        actix_web_lab::middleware::from_fn(crate::middleware::identify_grove::identify_grove)
    };
}

pub(crate) use grove;

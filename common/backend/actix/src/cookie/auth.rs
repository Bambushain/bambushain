use std::future::{ready, Ready};

use actix_web::{dev, FromRequest, HttpRequest};

use bamboo_common_core::error::*;

pub const BAMBOO_AUTH_COOKIE: &str = "BambooAuth";

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct BambooAuthCookie {
    pub token: String,
}

impl FromRequest for BambooAuthCookie {
    type Error = BambooError;
    type Future = Ready<Result<Self, BambooError>>;

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        ready(
            req.cookie(BAMBOO_AUTH_COOKIE)
                .ok_or(BambooError::unauthorized("user", "Auth cookie is not set"))
                .map(|cookie| Self {
                    token: cookie.value().to_string(),
                }),
        )
    }
}

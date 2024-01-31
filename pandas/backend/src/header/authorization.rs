use actix_web::{error, http::header, HttpMessage};

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

    fn parse<M: HttpMessage>(msg: &M) -> Result<Self, error::ParseError> {
        let authorization = if let Some(header) = msg.headers().get(header::AUTHORIZATION) {
            Ok(header)
        } else {
            Err(error::ParseError::Header)
        }?
        .to_str()
        .map_err(|_| error::ParseError::Header)
        .map(|header| header.strip_prefix("Panda ").map(|res| res.to_string()))?;

        Ok(AuthorizationHeader { authorization })
    }
}

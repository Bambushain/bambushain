use actix_web::{body, dev, Error};
use actix_web_lab::middleware::Next;

use bamboo_entities::bamboo_insufficient_rights_error;

use crate::middleware::authenticate_user::Authentication;

pub(crate) async fn check_mod(
    authentication_state: Option<Authentication>,
    req: dev::ServiceRequest,
    next: Next<impl body::MessageBody>,
) -> Result<dev::ServiceResponse<impl body::MessageBody>, Error> {
    if let Some(state) = authentication_state {
        if state.user.is_mod {
            next.call(req).await
        } else {
            Err(bamboo_insufficient_rights_error!(
                "user",
                "You need to be a mod"
            ).into())
        }
    } else {
        Err(bamboo_insufficient_rights_error!("user", "You need to be a mod").into())
    }
}

macro_rules! is_mod {
    () => {
        actix_web_lab::middleware::from_fn(crate::middleware::check_mod::check_mod)
    };
}

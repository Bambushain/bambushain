use std::future::{ready, Ready};
use std::rc::Rc;

use actix_web::{body, dev, web, Error, HttpMessage};
use futures_util::future::LocalBoxFuture;

use crate::DbConnection;
use pandaparty_dbal::prelude::*;
use pandaparty_entities::pandaparty_unauthorized_error;
use pandaparty_entities::prelude::*;

#[derive(Clone)]
pub struct AuthenticationState {
    pub token: String,
    pub user: User,
}

pub type Authentication = web::ReqData<AuthenticationState>;

pub struct AuthenticateUser;

impl<S: 'static, B> dev::Transform<S, dev::ServiceRequest> for AuthenticateUser
where
    S: dev::Service<dev::ServiceRequest, Response = dev::ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = dev::ServiceResponse<body::EitherBody<B>>;
    type Error = Error;
    type Transform = AuthenticateUserMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticateUserMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthenticateUserMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> dev::Service<dev::ServiceRequest> for AuthenticateUserMiddleware<S>
where
    S: dev::Service<dev::ServiceRequest, Response = dev::ServiceResponse<B>, Error = Error>
        + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = dev::ServiceResponse<body::EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, req: dev::ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {
            let unauthorized = unauthorized!(pandaparty_unauthorized_error!("", "No auth present"))
                .map_into_right_body();
            let request = req.request();

            let auth_header = match request.headers().get("Authorization") {
                Some(header) => header
                    .to_str()
                    .expect("Header value should be convertible to string"),
                _ => return Ok(dev::ServiceResponse::new(request.clone(), unauthorized)),
            };

            let token = match auth_header.strip_prefix("Panda ") {
                Some(token) => token,
                _ => return Ok(dev::ServiceResponse::new(request.clone(), unauthorized)),
            };

            let db = req.app_data::<DbConnection>().unwrap();
            let user = match get_user_by_token(token.to_string(), db).await {
                Ok(user) => user,
                _ => return Ok(dev::ServiceResponse::new(request.clone(), unauthorized)),
            };
            req.extensions_mut().insert(AuthenticationState {
                token: token.to_string(),
                user,
            });

            let res = svc.call(req).await;
            res.map(dev::ServiceResponse::map_into_left_body)
        })
    }
}

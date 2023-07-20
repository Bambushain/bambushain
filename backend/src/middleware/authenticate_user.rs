use std::future::{Ready, ready};
use std::rc::Rc;

use actix_web::{body, dev, Error, HttpMessage};
use futures_util::future::LocalBoxFuture;

use sheef_database::token::get_user_by_token;
use sheef_entities::sheef_unauthorized_error;
use sheef_entities::user::User;

pub struct AuthenticationState {
    pub token: String,
    pub user: User,
}

pub struct AuthenticateUser;

impl<S: 'static, B> dev::Transform<S, dev::ServiceRequest> for AuthenticateUser
    where
        S: dev::Service<dev::ServiceRequest, Response=dev::ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static, {
    type Response = dev::ServiceResponse<body::EitherBody<B>>;
    type Error = Error;
    type Transform = AuthenticateUserMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticateUserMiddleware {
            service: Rc::new(service)
        }))
    }
}

pub struct AuthenticateUserMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> dev::Service<dev::ServiceRequest> for AuthenticateUserMiddleware<S>
    where
        S: dev::Service<dev::ServiceRequest, Response=dev::ServiceResponse<B>, Error=Error> + 'static,
        S::Future: 'static,
        B: 'static, {
    type Response = dev::ServiceResponse<body::EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, req: dev::ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {
            let unauthorized = unauthorized!(sheef_unauthorized_error!("", "No auth present")).map_into_right_body();
            let request = req.request();

            let auth_header = match request.headers().get("Authorization") {
                Some(header) => header.to_str().expect("Header value should be convertible to string"),
                _ => return Ok(dev::ServiceResponse::new(request.clone(), unauthorized))
            };

            let token = match auth_header.strip_prefix("Sheef ") {
                Some(token) => token,
                _ => return Ok(dev::ServiceResponse::new(request.clone(), unauthorized))
            };
            let mut split_token = token.split('/');
            let username = match split_token.next() {
                Some(username) => username,
                _ => return Ok(dev::ServiceResponse::new(request.clone(), unauthorized))
            };
            let token = match split_token.next() {
                Some(token) => token,
                _ => return Ok(dev::ServiceResponse::new(request.clone(), unauthorized))
            };

            let user = match get_user_by_token(&username.to_string(), &token.to_string()).await {
                Ok(user) => user,
                _ => return Ok(dev::ServiceResponse::new(request.clone(), unauthorized))
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

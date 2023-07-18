use std::future::{Ready, ready};
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage};
use actix_web::error::ErrorUnauthorized;
use futures_util::future::LocalBoxFuture;
use sheef_database::token::get_user_by_token_sync;
use sheef_entities::user::User;

pub struct AuthenticationState {
    pub token: String,
    pub user: User,
}

pub struct AuthenticateUser;

impl<S, B> Transform<S, ServiceRequest> for AuthenticateUser where S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>, S::Future: 'static, B: 'static, {
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthenticateUserMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticateUserMiddleware { service }))
    }
}

pub struct AuthenticateUserMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticateUserMiddleware<S> where S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>, S::Future: 'static, B: 'static, {
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = match req.headers().get("Authorization") {
            Some(header) => header.to_str().expect("Header value should be convertible to string"),
            None => return Box::pin(async { Err(ErrorUnauthorized("no auth present")) })
        };

        if !auth_header.starts_with("Sheef") {
            return box_pin!(Err(ErrorUnauthorized("no auth present")));
        }

        let token = auth_header.strip_prefix("Sheef ").expect("Sheef should be appended");
        let mut split_token = token.split('/');
        let username = split_token.next().expect("Username should be present");
        let token = split_token.last().expect("Token should be present");

        let user = match get_user_by_token_sync(&username.to_string(), &token.to_string()) {
            Some(user) => user,
            None => return box_pin!(Err(ErrorUnauthorized("no auth present")))
        };
        req.extensions_mut().insert(AuthenticationState {
            token: token.to_string(),
            user,
        });

        let fut = self.service.call(req);
        box_pin!(fut.await)
    }
}

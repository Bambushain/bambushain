use std::future::{Ready, ready};
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage};
use actix_web::error::ErrorForbidden;
use futures_util::future::LocalBoxFuture;
use crate::middleware::authenticate_user::AuthenticationState;

pub struct CheckMod;

impl<S, B> Transform<S, ServiceRequest> for CheckMod where S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>, S::Future: 'static, B: 'static, {
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CheckModMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CheckModMiddleware { service }))
    }
}

pub struct CheckModMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CheckModMiddleware<S> where S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>, S::Future: 'static, B: 'static, {
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let is_mod = {
            let extensions = req.extensions();
            let state = extensions.get::<AuthenticationState>();
            if state.is_none() {
                return box_pin!(Err(ErrorForbidden("You need to be mod")));
            }
            state.unwrap().user.is_mod
        };
        if is_mod {
            let fut = self.service.call(req);
            box_pin!(fut.await)
        } else {
            box_pin!(Err(ErrorForbidden("You need to be mod")))
        }
    }
}
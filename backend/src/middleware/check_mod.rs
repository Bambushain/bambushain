use std::future::{Ready, ready};
use std::rc::Rc;

use actix_web::{body, Error, HttpMessage, HttpResponse};
use actix_web::dev;
use futures_util::future::LocalBoxFuture;

use crate::middleware::authenticate_user::AuthenticationState;

pub struct CheckMod;

impl<S: 'static, B> dev::Transform<S, dev::ServiceRequest> for CheckMod
    where
        S: dev::Service<dev::ServiceRequest, Response=dev::ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static, {
    type Response = dev::ServiceResponse<body::EitherBody<B>>;
    type Error = Error;
    type Transform = CheckModMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CheckModMiddleware {
            service: Rc::new(service)
        }))
    }
}

pub struct CheckModMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> dev::Service<dev::ServiceRequest> for CheckModMiddleware<S>
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
            let needs_to_be_mod = HttpResponse::Forbidden().json(pandaparty_entities::error::PandaPartyError { entity_type: "".to_string(), error_type: pandaparty_entities::error::PandaPartyErrorCode::InsufficientRightsError, message: "You need to be a mod".to_string() }).map_into_right_body();
            let request = req.request();

            let is_mod = {
                let extensions = req.extensions();
                let state = extensions.get::<AuthenticationState>();
                if state.is_none() {
                    return Ok(dev::ServiceResponse::new(request.clone(), needs_to_be_mod));
                }
                state.unwrap().user.is_mod
            };

            if is_mod {
                let res = svc.call(req).await;
                res.map(dev::ServiceResponse::map_into_left_body)
            } else {
                Ok(dev::ServiceResponse::new(request.clone(), needs_to_be_mod))
            }
        })
    }
}
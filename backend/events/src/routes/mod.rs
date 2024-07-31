use crate::sse;
use actix_web::web;
use bamboo_common::backend::services::{EnvService, EnvironmentService};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    let environment_service = EnvService::new(EnvironmentService::new());

    cfg.app_data(environment_service)
        .service(sse::event_sse_client);
}

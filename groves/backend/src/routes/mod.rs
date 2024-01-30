use actix_web::{web, HttpResponse};

use crate::middleware::authenticate_user::authenticate;
use bamboo_common::backend::services::{EnvService, EnvironmentService};

mod authentication;
mod groves;
mod user;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    let environment_service = EnvService::new(EnvironmentService::new());

    let frontend_base_path = environment_service.get_env("FRONTEND_DIR", ".");
    log::info!("Frontend base path: {frontend_base_path}");

    cfg.app_data(environment_service)
        .service(authentication::login)
        .service(authentication::login_callback)
        .service(authentication::logout)
        .route(
            "/api/login",
            web::head()
                .to(HttpResponse::NoContent)
                .wrap(authenticate!()),
        )
        .service(groves::get_groves)
        .service(groves::get_grove)
        .service(groves::create_grove)
        .service(groves::suspend_grove)
        .service(groves::resume_grove)
        .service(groves::delete_grove)
        .service(user::get_users)
        .service(user::reset_user_password)
        .service(user::make_user_mod)
        .service(
            actix_web_lab::web::spa()
                .index_file(format!("{frontend_base_path}/dist/index.html"))
                .static_resources_location(format!("{frontend_base_path}/dist"))
                .static_resources_mount("/static")
                .finish(),
        );
}

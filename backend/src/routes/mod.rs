mod authentication;
mod character;
mod character_housing;
mod crafter;
mod custom_field;
mod event;
mod fighter;
mod free_company;
mod my;
mod support;
mod user;

use actix_web::{web, HttpResponse};

use bamboo_services::prelude::{EnvService, EnvironmentService};

use crate::middleware::authenticate_user::authenticate;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    let environment_service = EnvService::new(EnvironmentService::new());

    let frontend_base_path = environment_service.get_env("FRONTEND_DIR", ".");
    log::info!("Frontend base path: {frontend_base_path}");

    cfg.app_data(environment_service)
        .service(authentication::login)
        .service(authentication::forgot_password)
        .service(authentication::logout)
        .route(
            "/api/login",
            web::head()
                .to(HttpResponse::NoContent)
                .wrap(authenticate!()),
        )
        .service(user::get_users)
        .service(user::create_user)
        .service(user::get_user)
        .service(user::delete_user)
        .service(user::update_user_profile)
        .service(user::add_mod_user)
        .service(user::remove_mod_user)
        .service(user::change_password)
        .service(event::get_events)
        .service(event::create_event)
        .service(event::update_event)
        .service(event::delete_event)
        .service(my::get_profile)
        .service(my::update_profile)
        .service(my::change_my_password)
        .service(my::enable_totp)
        .service(my::validate_totp)
        .service(custom_field::get_custom_fields)
        .service(custom_field::create_custom_field)
        .service(custom_field::get_custom_field)
        .service(custom_field::update_custom_field)
        .service(custom_field::move_custom_field)
        .service(custom_field::delete_custom_field)
        .service(custom_field::get_custom_field_options)
        .service(custom_field::create_custom_field_option)
        .service(custom_field::update_custom_field_option)
        .service(custom_field::delete_custom_field_option)
        .service(character::get_characters)
        .service(character::create_character)
        .service(character::get_character)
        .service(character::update_character)
        .service(character::delete_character)
        .service(free_company::get_free_companies)
        .service(free_company::create_free_company)
        .service(free_company::get_free_company)
        .service(free_company::update_free_company)
        .service(free_company::delete_free_company)
        .service(crafter::get_crafters)
        .service(crafter::create_crafter)
        .service(crafter::get_crafter)
        .service(crafter::update_crafter)
        .service(crafter::delete_crafter)
        .service(fighter::get_fighters)
        .service(fighter::create_fighter)
        .service(fighter::get_fighter)
        .service(fighter::update_fighter)
        .service(fighter::delete_fighter)
        .service(character_housing::get_character_housings)
        .service(character_housing::create_character_housing)
        .service(character_housing::get_character_housing)
        .service(character_housing::update_character_housing)
        .service(character_housing::delete_character_housing)
        .service(support::send_support_request)
        .service(support::report_glitchtip_error)
        .service(crate::sse::event::event_sse_client)
        .service(
            actix_web_lab::web::spa()
                .index_file(format!("{frontend_base_path}/dist/index.html"))
                .static_resources_location(format!("{frontend_base_path}/dist"))
                .static_resources_mount("/static")
                .finish(),
        );
}

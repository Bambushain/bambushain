mod authentication;
mod character;
mod crafter;
mod custom_field;
mod event;
mod fighter;
mod free_company;
mod user;

use actix_web::{web, HttpResponse};

use bamboo_services::prelude::{EnvService, EnvironmentService};

use crate::middleware::authenticate_user::authenticate;
use crate::routes::authentication::{login, logout};
use crate::routes::character::{
    create_character, delete_character, get_character, get_characters, update_character,
};
use crate::routes::crafter::{
    create_crafter, delete_crafter, get_crafter, get_crafters, update_crafter,
};
use crate::routes::custom_field::{
    create_custom_field, create_custom_field_option, delete_custom_field,
    delete_custom_field_option, get_custom_field, get_custom_field_options, get_custom_fields,
    move_custom_field, update_custom_field, update_custom_field_option,
};
use crate::routes::event::{create_event, delete_event, get_events, update_event};
use crate::routes::fighter::{
    create_fighter, delete_fighter, get_fighter, get_fighters, update_fighter,
};
use crate::routes::free_company::{
    create_free_company, delete_free_company, get_free_companies, get_free_company,
    update_free_company,
};
use crate::routes::user::{
    add_mod_user, change_my_password, change_password, create_user, delete_user, enable_totp,
    get_profile, get_user, get_users, remove_mod_user, update_profile, update_user_profile,
    validate_totp,
};
use crate::sse::event::event_sse_client;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    let environment_service = EnvService::new(EnvironmentService::new());

    let frontend_base_path = environment_service.get_env("FRONTEND_DIR", ".");
    log::info!("Frontend base path: {frontend_base_path}");

    cfg.app_data(environment_service)
        .service(login)
        .service(logout)
        .route(
            "/api/login",
            web::head()
                .to(HttpResponse::NoContent)
                .wrap(authenticate!()),
        )
        .service(get_users)
        .service(create_user)
        .service(get_user)
        .service(delete_user)
        .service(update_user_profile)
        .service(add_mod_user)
        .service(remove_mod_user)
        .service(change_password)
        .service(get_events)
        .service(create_event)
        .service(update_event)
        .service(delete_event)
        .service(get_profile)
        .service(update_profile)
        .service(change_my_password)
        .service(enable_totp)
        .service(validate_totp)
        .service(get_custom_fields)
        .service(create_custom_field)
        .service(get_custom_field)
        .service(update_custom_field)
        .service(move_custom_field)
        .service(delete_custom_field)
        .service(get_custom_field_options)
        .service(create_custom_field_option)
        .service(update_custom_field_option)
        .service(delete_custom_field_option)
        .service(get_characters)
        .service(create_character)
        .service(get_character)
        .service(update_character)
        .service(delete_character)
        .service(get_free_companies)
        .service(create_free_company)
        .service(get_free_company)
        .service(update_free_company)
        .service(delete_free_company)
        .service(get_crafters)
        .service(create_crafter)
        .service(get_crafter)
        .service(update_crafter)
        .service(delete_crafter)
        .service(get_fighters)
        .service(create_fighter)
        .service(get_fighter)
        .service(update_fighter)
        .service(delete_fighter)
        .service(event_sse_client)
        .service(
            actix_web_lab::web::spa()
                .index_file(format!("{frontend_base_path}/dist/index.html"))
                .static_resources_location(format!("{frontend_base_path}/dist"))
                .static_resources_mount("/static")
                .finish(),
        );
}

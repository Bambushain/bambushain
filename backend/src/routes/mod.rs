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
use crate::middleware::check_mod::is_mod;
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
        .route("/api/login", web::post().to(login))
        .route("/api/login", web::delete().to(logout).wrap(authenticate!()))
        .route(
            "/api/login",
            web::head()
                .to(HttpResponse::NoContent)
                .wrap(authenticate!()),
        )
        .route("/api/user", web::get().to(get_users).wrap(authenticate!()))
        .route(
            "/api/user",
            web::post()
                .to(create_user)
                .wrap(is_mod!())
                .wrap(authenticate!()),
        )
        .route(
            "/api/user/{id}",
            web::get().to(get_user).wrap(authenticate!()),
        )
        .route(
            "/api/user/{id}",
            web::delete()
                .to(delete_user)
                .wrap(is_mod!())
                .wrap(authenticate!()),
        )
        .route(
            "/api/user/{id}/profile",
            web::put()
                .to(update_user_profile)
                .wrap(is_mod!())
                .wrap(authenticate!()),
        )
        .route(
            "/api/user/{id}/mod",
            web::put()
                .to(add_mod_user)
                .wrap(is_mod!())
                .wrap(authenticate!()),
        )
        .route(
            "/api/user/{id}/mod",
            web::delete()
                .to(remove_mod_user)
                .wrap(is_mod!())
                .wrap(authenticate!()),
        )
        .route(
            "/api/user/{id}/password",
            web::put()
                .to(change_password)
                .wrap(is_mod!())
                .wrap(authenticate!()),
        )
        .route(
            "/api/pandaparty/event",
            web::get().to(get_events).wrap(authenticate!()),
        )
        .route(
            "/api/pandaparty/event",
            web::post().to(create_event).wrap(authenticate!()),
        )
        .route(
            "/api/pandaparty/event/{id}",
            web::put().to(update_event).wrap(authenticate!()),
        )
        .route(
            "/api/pandaparty/event/{id}",
            web::delete().to(delete_event).wrap(authenticate!()),
        )
        .route(
            "/api/bamboo-grove/event",
            web::get().to(get_events).wrap(authenticate!()),
        )
        .route(
            "/api/bamboo-grove/event",
            web::post().to(create_event).wrap(authenticate!()),
        )
        .route(
            "/api/bamboo-grove/event/{id}",
            web::put().to(update_event).wrap(authenticate!()),
        )
        .route(
            "/api/bamboo-grove/event/{id}",
            web::delete().to(delete_event).wrap(authenticate!()),
        )
        .route(
            "/api/my/profile",
            web::get().to(get_profile).wrap(authenticate!()),
        )
        .route(
            "/api/my/profile",
            web::put().to(update_profile).wrap(authenticate!()),
        )
        .route(
            "/api/my/password",
            web::put().to(change_my_password).wrap(authenticate!()),
        )
        .route(
            "/api/my/totp",
            web::post().to(enable_totp).wrap(authenticate!()),
        )
        .route(
            "/api/my/totp/validate",
            web::put().to(validate_totp).wrap(authenticate!()),
        )
        .route(
            "/api/final-fantasy/character/custom-field",
            web::get().to(get_custom_fields).wrap(authenticate!()),
        )
        .route(
            "/api/final-fantasy/character/custom-field",
            web::post().to(create_custom_field).wrap(authenticate!()),
        )
        .route(
            "/api/final-fantasy/character/custom-field/{id}",
            web::get().to(get_custom_field).wrap(authenticate!()),
        )
        .route(
            "/api/final-fantasy/character/custom-field/{id}",
            web::put().to(update_custom_field).wrap(authenticate!()),
        )
        .route(
            "/api/final-fantasy/character/custom-field/{field_id}/{position}",
            web::put().to(move_custom_field).wrap(authenticate!()),
        )
        .route(
            "/api/final-fantasy/character/custom-field/{id}",
            web::delete().to(delete_custom_field).wrap(authenticate!()),
        )
        .route(
            "/api/final-fantasy/character/custom-field/{id}/option",
            web::get()
                .to(get_custom_field_options)
                .wrap(authenticate!()),
        )
        .route(
            "/api/final-fantasy/character/custom-field/{id}/option",
            web::post()
                .to(create_custom_field_option)
                .wrap(authenticate!()),
        )
        .route(
            "/api/final-fantasy/character/custom-field/{field_id}/option/{id}",
            web::put()
                .to(update_custom_field_option)
                .wrap(authenticate!()),
        )
        .route(
            "/api/final-fantasy/character/custom-field/{field_id}/option/{id}",
            web::delete()
                .to(delete_custom_field_option)
                .wrap(authenticate!()),
        )
        .route(
            "/api/final-fantasy/character",
            web::get().to(get_characters).wrap(authenticate!()),
        )
        .route(
            "/api/final-fantasy/character",
            web::post().to(create_character).wrap(authenticate!()),
        )
        .route(
            "/api/final-fantasy/character/{id}",
            web::get().to(get_character).wrap(authenticate!()),
        )
        .route(
            "/api/final-fantasy/character/{id}",
            web::put().to(update_character).wrap(authenticate!()),
        )
        .route(
            "/api/final-fantasy/character/{id}",
            web::delete().to(delete_character).wrap(authenticate!()),
        )
        .route(
            "/api/final-fantasy/free-company",
            web::get().to(get_free_companies).wrap(authenticate!()),
        )
        .route(
            "/api/final-fantasy/free-company",
            web::post().to(create_free_company).wrap(authenticate!()),
        )
        .route(
            "/api/final-fantasy/free-company/{id}",
            web::get().to(get_free_company).wrap(authenticate!()),
        )
        .route(
            "/api/final-fantasy/free-company/{id}",
            web::put().to(update_free_company).wrap(authenticate!()),
        )
        .route(
            "/api/final-fantasy/free-company/{id}",
            web::delete().to(delete_free_company).wrap(authenticate!()),
        )
        .route(
            "/api/final-fantasy/character/{character_id}/crafter",
            web::get().to(get_crafters).wrap(authenticate!()),
        )
        .route(
            "/api/final-fantasy/character/{character_id}/crafter",
            web::post().to(create_crafter).wrap(authenticate!()),
        )
        .route(
            "/api/final-fantasy/character/{character_id}/crafter/{id}",
            web::get().to(get_crafter).wrap(authenticate!()),
        )
        .route(
            "/api/final-fantasy/character/{character_id}/crafter/{id}",
            web::put().to(update_crafter).wrap(authenticate!()),
        )
        .route(
            "/api/final-fantasy/character/{character_id}/crafter/{id}",
            web::delete().to(delete_crafter).wrap(authenticate!()),
        )
        .route(
            "/api/final-fantasy/character/{character_id}/fighter",
            web::get().to(get_fighters).wrap(authenticate!()),
        )
        .route(
            "/api/final-fantasy/character/{character_id}/fighter",
            web::post().to(create_fighter).wrap(authenticate!()),
        )
        .route(
            "/api/final-fantasy/character/{character_id}/fighter/{id}",
            web::get().to(get_fighter).wrap(authenticate!()),
        )
        .route(
            "/api/final-fantasy/character/{character_id}/fighter/{id}",
            web::put().to(update_fighter).wrap(authenticate!()),
        )
        .route(
            "/api/final-fantasy/character/{character_id}/fighter/{id}",
            web::delete().to(delete_fighter).wrap(authenticate!()),
        )
        .route("/sse/event", web::get().to(event_sse_client))
        .service(
            actix_web_lab::web::spa()
                .index_file(format!("{frontend_base_path}/dist/index.html"))
                .static_resources_location(format!("{frontend_base_path}/dist"))
                .static_resources_mount("/static")
                .finish(),
        );
}

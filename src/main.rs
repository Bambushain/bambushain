use std::sync::Arc;

use actix_web::{App, HttpResponse, HttpServer};
use actix_web::web;
use rand::distributions::Alphanumeric;
use rand::prelude::*;
use sea_orm::prelude::*;

use pandaparty_backend::{DbConnection, Services, ServicesState};
use pandaparty_backend::broadcaster::event::EventBroadcaster;
use pandaparty_backend::broadcaster::user::UserBroadcaster;
use pandaparty_backend::middleware::authenticate_user::AuthenticateUser;
use pandaparty_backend::middleware::check_mod::CheckMod;
use pandaparty_backend::routes::authentication::{login, logout};
use pandaparty_backend::routes::character::{create_character, delete_character, get_character, get_characters, update_character};
use pandaparty_backend::routes::crafter::{create_crafter, delete_crafter, get_crafter, get_crafters, update_crafter};
use pandaparty_backend::routes::custom_field::{create_custom_field, create_custom_field_option, delete_custom_field, delete_custom_field_option, get_custom_field, get_custom_field_options, get_custom_fields, update_custom_field, update_custom_field_option};
use pandaparty_backend::routes::event::{create_event, delete_event, get_events, update_event};
use pandaparty_backend::routes::fighter::{create_fighter, delete_fighter, get_fighter, get_fighters, update_fighter};
use pandaparty_backend::routes::user::{add_mod_user, change_my_password, change_password, create_user, delete_user, enable_totp, get_profile, get_user, get_users, remove_mod_user, update_profile, update_user_profile, validate_totp};
use pandaparty_backend::sse::{Notification, NotificationState};
use pandaparty_backend::sse::event::event_sse_client;
use pandaparty_backend::sse::user::user_sse_client;
use pandaparty_entities::user;
use pandaparty_migration::{IntoSchemaManagerConnection, Migrator, MigratorTrait};
use pandaparty_services::prelude::EnvironmentService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    stderrlog::new()
        .verbosity(log::Level::Info)
        .init()
        .unwrap();

    log::info!("Start the Pandaparty");

    let mut opts = sea_orm::ConnectOptions::new(std::env::var("DATABASE_URL").expect("Needs DATABASE_URL"));
    opts.sqlx_logging(true).sqlx_logging_level(log::LevelFilter::Debug);

    let db = match sea_orm::Database::connect(opts).await {
        Ok(db) => db,
        Err(err) => panic!("{err}")
    };

    match Migrator::up(db.into_schema_manager_connection(), None).await {
        Ok(_) => log::info!("Successfully migrated database"),
        Err(err) => panic!("{err}")
    }

    let at_least_one_mod_exists = match user::Entity::find()
        .filter(user::Column::IsMod.eq(true))
        .count(&db)
        .await {
        Ok(count) => {
            log::info!("Database contains {count} users");
            count > 0
        }
        Err(err) => panic!("{err}")
    };

    if !at_least_one_mod_exists {
        log::info!("At least one user exists, not creating initial user");
        let password = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(12)
            .map(char::from)
            .collect::<String>();

        let display_name = std::env::var("INITIAL_USER_DISPLAY_NAME").expect("INITIAL_USER_DISPLAY_NAME must be set");
        let email = std::env::var("INITIAL_USER_EMAIL").expect("INITIAL_USER_EMAIL must be set");

        match pandaparty_dbal::user::create_user(user::Model::new(email.clone(), password.clone(), display_name.clone(), "".into(), true), &db).await {
            Ok(_) => log::info!("Created initial user {email} with password {password}"),
            Err(err) => panic!("Failed to create initial user, {err}")
        }
    }

    let user_broadcaster = UserBroadcaster::create();
    let event_broadcaster = EventBroadcaster::create();

    let environment_service = EnvironmentService::new();

    let frontend_base_path = environment_service.get_env("FRONTEND_DIR", ".");
    log::info!("Frontend base path: {frontend_base_path}");

    log::info!("Serving on port 8070");
    HttpServer::new(move || {
        App::new()
            .app_data(Notification::new(NotificationState {
                user_broadcaster: Arc::clone(&user_broadcaster),
                event_broadcaster: Arc::clone(&event_broadcaster),
            }))
            .app_data(Services::new(ServicesState {
                environment_service: Arc::new(environment_service.clone()),
            }))
            .app_data(DbConnection::new(db.clone()))

            .route("/api/login", web::post().to(login))
            .route("/api/login", web::delete().to(logout).wrap(AuthenticateUser))
            .route("/api/login", web::head().to(HttpResponse::NoContent).wrap(AuthenticateUser))

            .route("/api/user", web::get().to(get_users).wrap(AuthenticateUser))
            .route("/api/user", web::post().to(create_user).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user/{id}", web::get().to(get_user).wrap(AuthenticateUser))
            .route("/api/user/{id}", web::delete().to(delete_user).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user/{id}/profile", web::put().to(update_user_profile).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user/{id}/mod", web::put().to(add_mod_user).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user/{id}/mod", web::delete().to(remove_mod_user).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user/{id}/password", web::put().to(change_password).wrap(CheckMod).wrap(AuthenticateUser))

            .route("/api/pandaparty/event", web::get().to(get_events).wrap(AuthenticateUser))
            .route("/api/pandaparty/event", web::post().to(create_event).wrap(AuthenticateUser))
            .route("/api/pandaparty/event/{id}", web::put().to(update_event).wrap(AuthenticateUser))
            .route("/api/pandaparty/event/{id}", web::delete().to(delete_event).wrap(AuthenticateUser))

            .route("/api/my/profile", web::get().to(get_profile).wrap(AuthenticateUser))
            .route("/api/my/profile", web::put().to(update_profile).wrap(AuthenticateUser))
            .route("/api/my/password", web::put().to(change_my_password).wrap(AuthenticateUser))
            .route("/api/my/totp", web::post().to(enable_totp).wrap(AuthenticateUser))
            .route("/api/my/totp/validate", web::put().to(validate_totp).wrap(AuthenticateUser))

            .route("/api/final-fantasy/character/custom-field", web::get().to(get_custom_fields).wrap(AuthenticateUser))
            .route("/api/final-fantasy/character/custom-field", web::post().to(create_custom_field).wrap(AuthenticateUser))
            .route("/api/final-fantasy/character/custom-field/{id}", web::get().to(get_custom_field).wrap(AuthenticateUser))
            .route("/api/final-fantasy/character/custom-field/{id}", web::put().to(update_custom_field).wrap(AuthenticateUser))
            .route("/api/final-fantasy/character/custom-field/{id}", web::delete().to(delete_custom_field).wrap(AuthenticateUser))

            .route("/api/final-fantasy/character/custom-field/{id}/option", web::get().to(get_custom_field_options).wrap(AuthenticateUser))
            .route("/api/final-fantasy/character/custom-field/{id}/option", web::post().to(create_custom_field_option).wrap(AuthenticateUser))
            .route("/api/final-fantasy/character/custom-field/{field_id}/option/{id}", web::put().to(update_custom_field_option).wrap(AuthenticateUser))
            .route("/api/final-fantasy/character/custom-field/{field_id}/option/{id}", web::delete().to(delete_custom_field_option).wrap(AuthenticateUser))

            .route("/api/final-fantasy/character", web::get().to(get_characters).wrap(AuthenticateUser))
            .route("/api/final-fantasy/character", web::post().to(create_character).wrap(AuthenticateUser))
            .route("/api/final-fantasy/character/{id}", web::get().to(get_character).wrap(AuthenticateUser))
            .route("/api/final-fantasy/character/{id}", web::put().to(update_character).wrap(AuthenticateUser))
            .route("/api/final-fantasy/character/{id}", web::delete().to(delete_character).wrap(AuthenticateUser))

            .route("/api/final-fantasy/character/{character_id}/crafter", web::get().to(get_crafters).wrap(AuthenticateUser))
            .route("/api/final-fantasy/character/{character_id}/crafter", web::post().to(create_crafter).wrap(AuthenticateUser))
            .route("/api/final-fantasy/character/{character_id}/crafter/{id}", web::get().to(get_crafter).wrap(AuthenticateUser))
            .route("/api/final-fantasy/character/{character_id}/crafter/{id}", web::put().to(update_crafter).wrap(AuthenticateUser))
            .route("/api/final-fantasy/character/{character_id}/crafter/{id}", web::delete().to(delete_crafter).wrap(AuthenticateUser))

            .route("/api/final-fantasy/character/{character_id}/fighter", web::get().to(get_fighters).wrap(AuthenticateUser))
            .route("/api/final-fantasy/character/{character_id}/fighter", web::post().to(create_fighter).wrap(AuthenticateUser))
            .route("/api/final-fantasy/character/{character_id}/fighter/{id}", web::get().to(get_fighter).wrap(AuthenticateUser))
            .route("/api/final-fantasy/character/{character_id}/fighter/{id}", web::put().to(update_fighter).wrap(AuthenticateUser))
            .route("/api/final-fantasy/character/{character_id}/fighter/{id}", web::delete().to(delete_fighter).wrap(AuthenticateUser))

            .route("/sse/user", web::get().to(user_sse_client))
            .route("/sse/event", web::get().to(event_sse_client))

            .service(
                actix_web_lab::web::spa()
                    .index_file(format!("{frontend_base_path}/dist/index.html"))
                    .static_resources_location(format!("{frontend_base_path}/dist"))
                    .static_resources_mount("/static")
                    .finish()
            )
    })
        .bind(("0.0.0.0", 8070))?
        .run()
        .await
}

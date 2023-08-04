use std::sync::Arc;

use actix_web::{App, HttpResponse, HttpServer};
use actix_web::web;

use pandaparty_backend::broadcaster::crew::CrewBroadcaster;
use pandaparty_backend::DbConnection;
use pandaparty_backend::middleware::authenticate_user::AuthenticateUser;
use pandaparty_backend::middleware::check_mod::CheckMod;
use pandaparty_backend::routes::authentication::{login, logout};
use pandaparty_backend::routes::crafter::{create_crafter, delete_crafter, get_crafter, get_crafters, update_crafter};
use pandaparty_backend::routes::fighter::{create_fighter, delete_fighter, get_fighter, get_fighters, update_fighter};
use pandaparty_backend::routes::user::{add_mod_user, change_my_password, change_password, create_user, delete_user, get_profile, get_user, get_users, remove_mod_user, update_profile, update_user_profile};
use pandaparty_backend::sse::crew::crew_sse_client;
use pandaparty_backend::sse::NotificationState;
use pandaparty_migration::{IntoSchemaManagerConnection, Migrator, MigratorTrait};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    stderrlog::new()
        .verbosity(log::Level::Info)
        .init()
        .unwrap();

    log::info!("Start the Pandaparty");

    let mut opts = sea_orm::ConnectOptions::new(std::env::var("DATABASE_URL").expect("Needs DATABASE_URL"));
    opts.sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Debug);

    let db = match sea_orm::Database::connect(opts).await {
        Ok(db) => db,
        Err(err) => panic!("{err}")
    };

    match Migrator::up(db.into_schema_manager_connection(), None).await {
        Ok(_) => log::info!("Successfully migrated database"),
        Err(err) => panic!("{err}")
    }

    let crew_broadcaster = CrewBroadcaster::create();

    let base_path = std::env::var("FRONTEND_DIR").unwrap_or(".".to_string());
    log::info!("Frontend base path: {base_path}");

    log::info!("Serving on port 8070");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(NotificationState {
                crew_broadcaster: Arc::clone(&crew_broadcaster),
            }))
            .app_data(DbConnection::new(db.clone()))

            .route("/api/login", web::post().to(login))
            .route("/api/login", web::delete().to(logout).wrap(AuthenticateUser))
            .route("/api/login", web::head().to(HttpResponse::NoContent).wrap(AuthenticateUser))

            .route("/api/user", web::get().to(get_users).wrap(AuthenticateUser))
            .route("/api/user", web::post().to(create_user).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user/{username}", web::get().to(get_user).wrap(AuthenticateUser))
            .route("/api/user/{username}", web::delete().to(delete_user).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user/{username}/profile", web::put().to(update_user_profile).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user/{username}/mod", web::put().to(add_mod_user).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user/{username}/mod", web::delete().to(remove_mod_user).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user/{username}/password", web::put().to(change_password).wrap(CheckMod).wrap(AuthenticateUser))

            .route("/api/crafter", web::get().to(get_crafters).wrap(AuthenticateUser))
            .route("/api/crafter", web::post().to(create_crafter).wrap(AuthenticateUser))
            .route("/api/crafter/{job}", web::get().to(get_crafter).wrap(AuthenticateUser))
            .route("/api/crafter/{job}", web::put().to(update_crafter).wrap(AuthenticateUser))
            .route("/api/crafter/{job}", web::delete().to(delete_crafter).wrap(AuthenticateUser))

            .route("/api/fighter", web::get().to(get_fighters).wrap(AuthenticateUser))
            .route("/api/fighter", web::post().to(create_fighter).wrap(AuthenticateUser))
            .route("/api/fighter/{job}", web::get().to(get_fighter).wrap(AuthenticateUser))
            .route("/api/fighter/{job}", web::put().to(update_fighter).wrap(AuthenticateUser))
            .route("/api/fighter/{job}", web::delete().to(delete_fighter).wrap(AuthenticateUser))

            .route("/api/my/profile", web::get().to(get_profile).wrap(AuthenticateUser))
            .route("/api/my/profile", web::put().to(update_profile).wrap(AuthenticateUser))
            .route("/api/my/password", web::put().to(change_my_password).wrap(AuthenticateUser))

            .route("/sse/crew", web::get().to(crew_sse_client))

            .service(
                actix_web_lab::web::spa()
                    .index_file(format!("{base_path}/dist/index.html"))
                    .static_resources_location(format!("{base_path}/dist"))
                    .static_resources_mount("/static")
                    .finish()
            )
    })
        .bind(("0.0.0.0", 8070))?
        .run()
        .await
}
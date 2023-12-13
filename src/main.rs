use actix_web::{middleware, App, HttpServer};
use rand::distributions::Alphanumeric;
use rand::prelude::*;
use sea_orm::prelude::*;

use bamboo_backend::prelude::*;
use bamboo_dbal::prelude::dbal;
use bamboo_entities::user;
use bamboo_migration::{IntoSchemaManagerConnection, Migrator, MigratorTrait};
use bamboo_services::prelude::DbConnection;

fn main() -> std::io::Result<()> {
    let mut log_builder = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"));
    let logger = sentry_log::SentryLogger::with_dest(log_builder.build());

    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(log::LevelFilter::Info);

    log::info!("Configure glitchtip");
    let _sentry = sentry::init(sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    });

    actix_web::rt::System::new().block_on(async {
        log::info!("Open the bamboo grove");

        let mut opts = sea_orm::ConnectOptions::new(
            std::env::var("DATABASE_URL").expect("Needs DATABASE_URL"),
        );
        opts.sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Debug);

        let db = sea_orm::Database::connect(opts)
            .await
            .map_err(std::io::Error::other)?;

        Migrator::up(db.into_schema_manager_connection(), None)
            .await
            .map_err(std::io::Error::other)?;
        log::info!("Successfully migrated database");

        let at_least_one_mod_exists = user::Entity::find()
            .filter(user::Column::IsMod.eq(true))
            .count(&db)
            .await
            .map(|count| count > 0)
            .map_err(std::io::Error::other)?;

        if !at_least_one_mod_exists {
            log::info!("No mod exists, creating initial user");
            let password = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(12)
                .map(char::from)
                .collect::<String>();

            let display_name = std::env::var("INITIAL_USER_DISPLAY_NAME")
                .expect("INITIAL_USER_DISPLAY_NAME must be set");
            let email =
                std::env::var("INITIAL_USER_EMAIL").expect("INITIAL_USER_EMAIL must be set");

            let _ = dbal::create_user(
                user::Model::new(
                    email.clone(),
                    password.clone(),
                    display_name.clone(),
                    "".into(),
                    true,
                ),
                &db,
            )
            .await
            .map_err(std::io::Error::other)?;
            log::info!("Created initial user {email} with password {password}");
        }

        let notifier = NotifierState::new();

        HttpServer::new(move || {
            App::new()
                .wrap(sentry_actix::Sentry::new())
                .wrap(middleware::Compress::default())
                .app_data(Notifier::new(notifier.clone()))
                .app_data(DbConnection::new(db.clone()))
                .configure(bamboo_backend::prelude::configure_routes)
        })
        .bind(("0.0.0.0", 8070))?
        .run()
        .await
    })?;
    Ok(())
}

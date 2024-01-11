use std::sync::Arc;

use actix_web::{middleware, App, HttpServer};
use sea_orm::{ConnectOptions, Database};

use bamboo_backend::prelude::*;
use bamboo_dbal::prelude::dbal;
use bamboo_migration::{IntoSchemaManagerConnection, Migrator, MigratorTrait};
use bamboo_services::prelude::DbConnection;

async fn setup_google_playstore_grove(
    db: &sea_orm::DatabaseConnection,
) -> std::io::Result<bamboo_entities::prelude::Grove> {
    if let Ok(grove) = dbal::get_grove_by_name("Google".to_string(), db).await {
        Ok(grove)
    } else {
        dbal::create_grove("Google".to_string(), db)
            .await
            .map_err(std::io::Error::other)
    }
}

async fn setup_google_playstore_user(db: &sea_orm::DatabaseConnection) -> std::io::Result<()> {
    let email = "playstore@google.bambushain".to_string();
    let password = "NkWHoLDmzg4aVEx".to_string();

    if let Ok(user) = dbal::get_user_by_email_or_username(email.clone(), db).await {
        dbal::change_password(user.grove_id, user.id, password, db)
            .await
            .map_err(std::io::Error::other)
            .map(|_| ())
    } else {
        let grove = setup_google_playstore_grove(db).await?;
        dbal::create_user(
            grove.id,
            bamboo_entities::prelude::User::new(
                email,
                "Google Playstore".to_string(),
                "google".to_string(),
                true,
            ),
            password,
            db,
        )
        .await
        .map_err(std::io::Error::other)
        .map(|_| ())
    }
}

fn main() -> std::io::Result<()> {
    let mut log_builder =
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"));
    let logger = sentry::integrations::log::SentryLogger::with_dest(log_builder.build());

    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(log::LevelFilter::Info);

    log::info!("Configure glitchtip");
    let _sentry = sentry::init(sentry::ClientOptions {
        release: sentry::release_name!(),
        before_send: Some(Arc::new(|mut event| {
            if let Some(mut request) = event.request {
                request.cookies = None;
                let _ = request.headers.remove_entry("authorization");
                let _ = request.headers.remove_entry("x-forwarded-for");
                let _ = request.headers.remove_entry("x-forwarded-host");
                let _ = request.headers.remove_entry("x-forwarded-proto");
                let _ = request.headers.remove_entry("x-forwarded-server");
                let _ = request.headers.remove_entry("x-real-ip");
                let _ = request.headers.remove_entry("cookie");
                event.request = Some(request);
            };
            Some(event)
        })),
        attach_stacktrace: true,
        auto_session_tracking: true,
        session_mode: sentry::SessionMode::Request,
        ..sentry::ClientOptions::default()
    });

    actix_web::rt::System::new().block_on(async {
        log::info!("Open the bamboo grove");

        let mut opts =
            ConnectOptions::new(std::env::var("DATABASE_URL").expect("Needs DATABASE_URL"));
        opts.sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Debug);

        let db = Database::connect(opts)
            .await
            .map_err(std::io::Error::other)?;

        Migrator::up(db.into_schema_manager_connection(), None)
            .await
            .map_err(std::io::Error::other)?;
        log::info!("Successfully migrated database");
        let groves = dbal::get_groves(&db).await.map_err(std::io::Error::other)?;

        if groves.is_empty()
            || groves
                .iter()
                .filter(|grove| grove.name == "Google".to_string())
                .count()
                == groves.len()
        {
            log::info!("Create initial grove as it doesn't exist");
            let initial_grove = dbal::create_grove(
                std::env::var("INITIAL_GROVE").expect("Needs INITIAL_GROVE"),
                &db,
            )
            .await
            .map_err(std::io::Error::other)?;

            log::info!("Migrate existing users and events to the new grove");
            dbal::migrate_between_groves(None, initial_grove.id, &db)
                .await
                .map_err(std::io::Error::other)?;
        }

        setup_google_playstore_user(&db).await?;

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

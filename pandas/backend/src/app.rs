use actix_web::{middleware, App, HttpServer};
use bamboo_common::backend::dbal;
use bamboo_common::backend::migration::{Migrator, MigratorTrait};
use bamboo_common::backend::services::minio_service::MinioClient;
use bamboo_common::backend::services::DbConnection;

use crate::notifier;
use crate::routes;

async fn setup_google_playstore_grove(
    user_id: i32,
    db: &sea_orm::DatabaseConnection,
) -> std::io::Result<()> {
    if !(dbal::grove_exists_by_name("Google".to_string(), db)
        .await
        .map_err(std::io::Error::other)?)
    {
        dbal::create_grove("Google".to_string(), false, user_id, db)
            .await
            .map_err(std::io::Error::other)
            .map(|_| ())
    } else {
        Ok(())
    }
}

async fn setup_google_playstore_user(db: &sea_orm::DatabaseConnection) -> std::io::Result<()> {
    let email = "playstore@google.bambushain".to_string();
    let password = "NkWHoLDmzg4aVEx".to_string();

    if let Ok(user) = dbal::get_user_by_email_or_username(email.clone(), db).await {
        dbal::set_password(user.id, password, db)
            .await
            .map_err(std::io::Error::other)
            .map(|_| ())
    } else {
        let user = dbal::create_user(
            bamboo_common::core::entities::User::new(
                email,
                "Google Playstore".to_string(),
                "google".to_string(),
            ),
            password,
            db,
        )
        .await
        .map_err(std::io::Error::other)?;
        setup_google_playstore_grove(user.id, db).await
    }
}

pub fn start_server() -> std::io::Result<()> {
    env_logger::init();

    actix_web::rt::System::new().block_on(async {
        log::info!("Open the bamboo grove");
        let db = bamboo_common::backend::database::get_database()
            .await
            .map_err(std::io::Error::other)?;

        let migrations = Migrator::get_pending_migrations(&db)
            .await
            .map_err(std::io::Error::other)?;
        log::info!("Running {} migrations", migrations.len());

        Migrator::up(&db, None)
            .await
            .map_err(std::io::Error::other)?;
        log::info!("Successfully migrated database");
        let minio_client = MinioClient::new(
            std::env::var("S3_BUCKET").map_err(std::io::Error::other)?,
            std::env::var("S3_ACCESS_KEY").map_err(std::io::Error::other)?,
            std::env::var("S3_SECRET_KEY").map_err(std::io::Error::other)?,
            std::env::var("S3_REGION").map_err(std::io::Error::other)?,
            std::env::var("S3_ENDPOINT").ok(),
            std::env::var("S3_USE_PATH_STYLE")
                .ok()
                .map_or(false, |val| val.to_lowercase() == "true"),
        )
        .map_err(std::io::Error::other)?;

        setup_google_playstore_user(&db).await?;

        let notifier = notifier::NotifierState::new();

        HttpServer::new(move || {
            App::new()
                .wrap(middleware::Compress::default())
                .app_data(bamboo_common::backend::services::MinioService::new(
                    minio_client.clone(),
                ))
                .app_data(notifier::Notifier::new(notifier.clone()))
                .app_data(DbConnection::new(db.clone()))
                .configure(routes::configure_routes)
        })
        .bind(("0.0.0.0", 8070))?
        .run()
        .await
    })?;
    Ok(())
}

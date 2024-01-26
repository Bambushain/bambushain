use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::{middleware, App, HttpServer};

use bamboo_common::backend::services::DbConnection;
use bamboo_common::core::error::BambooError;

use crate::routes;

pub async fn start_server() -> std::io::Result<()> {
    env_logger::init();

    log::info!("Open the grove management");
    let db = bamboo_common::backend::database::get_database()
        .await
        .map_err(std::io::Error::other)?;
    let key = std::env::var("ENCRYPTION_KEY")
        .or(Err(BambooError::unknown(
            "startup",
            "Invalid configuration",
        )))
        .map_err(std::io::Error::other)?;
    let secret_key = Key::from(key.as_bytes());

    HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_name("groves-session".into())
                    .build(),
            )
            .wrap(middleware::Compress::default())
            .app_data(DbConnection::new(db.clone()))
            .configure(routes::configure_routes)
    })
    .bind(("0.0.0.0", 8070))?
    .run()
    .await
}

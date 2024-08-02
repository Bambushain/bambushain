use crate::listener::start_listening;
use crate::notifier::{Notifier, NotifierState};
use crate::routes;
use actix_web::{middleware, App, HttpServer};
use bamboo_common::backend::services::DbConnection;

pub async fn start_server() -> std::io::Result<()> {
    bamboo_common::backend::logging::init();

    log::info!("Listening for sse connections");
    let db = bamboo_common::backend::database::get_database()
        .await
        .map_err(std::io::Error::other)?;

    let notifier_state = NotifierState::new();

    let (handle, stop_signal) = start_listening(notifier_state.clone())
        .await
        .map_err(std::io::Error::other)?;

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .app_data(DbConnection::new(db.clone()))
            .app_data(Notifier::new(notifier_state.clone()))
            .configure(routes::configure_routes)
    })
    .bind(("0.0.0.0", 4020))?
    .run()
    .await?;

    stop_signal.cancel();

    handle.await.map_err(std::io::Error::other)?;

    Ok(())
}

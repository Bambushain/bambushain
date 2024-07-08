use actix_web::{middleware, App, HttpServer};

use bamboo_common::backend::services::DbConnection;

use crate::routes;

pub async fn start_server() -> std::io::Result<()> {
    env_logger::init();

    log::info!("Open the grove management");
    let db = bamboo_common::backend::database::get_database()
        .await
        .map_err(std::io::Error::other)?;

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .app_data(DbConnection::new(db.clone()))
            .configure(routes::configure_routes)
    })
    .bind(("0.0.0.0", 8070))?
    .run()
    .await
}

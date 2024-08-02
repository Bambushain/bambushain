use actix_web::{App, HttpServer};

pub async fn start_server() -> std::io::Result<()> {
    bamboo_common::backend::logging::init();
    let frontend_base_path = std::env::var("FRONTEND_DIR").unwrap_or(".".to_string());
    log::info!("Frontend base path: {frontend_base_path}");

    log::info!("Starting pandas ui");
    HttpServer::new(move || {
        App::new().service(
            actix_web_lab::web::spa()
                .index_file(format!("{frontend_base_path}/index.html"))
                .static_resources_location(format!("{frontend_base_path}"))
                .static_resources_mount("/pandas/static")
                .finish(),
        )
    })
    .bind(("0.0.0.0", 4080))?
    .run()
    .await
}

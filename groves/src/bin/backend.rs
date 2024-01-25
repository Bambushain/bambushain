#[actix_web::main]
async fn main() -> std::io::Result<()> {
    bamboo_groves_backend::start_server().await
}

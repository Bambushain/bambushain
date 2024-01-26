#[actix_web::main]
async fn main() -> std::io::Result<()> {
    bamboo_groves::backend::start_server().await
}

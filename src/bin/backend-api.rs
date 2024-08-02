#[actix::main]
async fn main() -> std::io::Result<()> {
    bamboo_backend::api::start_server().await
}

#[actix::main]
async fn main() -> std::io::Result<()> {
    bamboo_backend::events::start_server().await
}

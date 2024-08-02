#[actix::main]
async fn main() -> std::io::Result<()> {
    bamboo_backend::pandas::start_server().await
}

#[actix::main]
async fn main() -> std::io::Result<()> {
    bamboo_pandas::events::start_server().await
}

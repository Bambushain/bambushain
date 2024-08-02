#[actix::main]
async fn main() -> std::io::Result<()> {
    bamboo_backend::mailer::start().await
}

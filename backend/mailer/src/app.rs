use crate::listener;

pub async fn start() -> std::io::Result<()> {
    bamboo_common::backend::logging::init();

    log::info!("Listening for new mails to send");
    listener::start_listening()
        .await
        .map_err(std::io::Error::other)
}

use crate::mailer;
use bamboo_common::backend::mailing::Mail;
use bamboo_common::backend::mq::{get_nats, FromMessage, NotificationError, Queue};
use bamboo_common::backend::services::EnvironmentService;
use futures_util::StreamExt;

pub async fn start_listening() -> Result<(), NotificationError> {
    log::info!("Start listening to new mails on nats");
    let nats_client = get_nats().await?;

    let mut subscriber = nats_client
        .subscribe(Queue::Mails)
        .await
        .map_err(|err| NotificationError::new(err.to_string()))?;

    while let Some(message) = subscriber.next().await {
        if let Ok(mail) = Mail::from_message(message) {
            if let Err(err) = mailer::send_mail(mail, EnvironmentService::new()).await {
                log::error!("Failed to send email: {err}");
            }
        }
    }

    Ok(())
}

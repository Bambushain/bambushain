use crate::mailer;
use async_nats::jetstream::consumer::pull;
use async_nats::subject::ToSubject;
use bamboo_common::backend::mailing::Mail;
use bamboo_common::backend::mq::{get_once_stream, FromMessage, NotificationError, Queue};
use bamboo_common::backend::services::EnvironmentService;
use futures_util::StreamExt;

pub async fn start_listening() -> Result<(), NotificationError> {
    log::info!("Start listening to new mails on nats");
    let consumer = get_once_stream()
        .await?
        .get_or_create_consumer(
            "mailing",
            pull::Config {
                filter_subject: Queue::Mails.to_subject().to_string(),
                ..Default::default()
            },
        )
        .await
        .map_err(|err| NotificationError::new(format!("Failed to get consumer {err}")))?;

    loop {
        let mut subscriber = consumer
            .fetch()
            .messages()
            .await
            .map_err(|_| NotificationError::new("Failed to start fetching"))?;

        while let Some(message) = subscriber.next().await {
            if let Ok(message) = message {
                if let Err(err) = message
                    .double_ack()
                    .await
                    .map_err(|err| NotificationError::new(format!("Failed to ack {err}")))
                {
                    log::error!("Failed to ack message {err}")
                } else {
                    if let Ok(mail) = Mail::from_jetstream_message(message) {
                        if let Err(err) = mailer::send_mail(mail, EnvironmentService::new()).await {
                            log::error!("Failed to send email: {err}");
                        }
                    }
                }
            } else if let Err(err) = message {
                log::error!("Failed to receive message {err}")
            }
        }
    }
}

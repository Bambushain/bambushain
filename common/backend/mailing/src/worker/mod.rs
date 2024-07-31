use crate::Mail;
use bamboo_common_backend_mq::{publish, Queue};

pub async fn enqueue_mail(mail: Mail) {
    if let Err(err) = publish(Queue::Mails, mail).await {
        log::error!("Failed to enqueue mail: {err}")
    }
}

use crate::Mail;
use bamboo_common_backend_mq::{publish_once, Queue};

pub async fn enqueue_mail(mail: Mail) {
    if let Err(err) = publish_once(Queue::Mails, mail).await {
        log::error!("Failed to enqueue mail: {err}")
    }
}

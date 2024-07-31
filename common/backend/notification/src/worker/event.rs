use crate::models::EventAction;
use bamboo_common_backend_mq::{publish, Queue};

pub async fn enqueue_event(action: EventAction) {
    if let Err(err) = publish(Queue::Events, action).await {
        log::error!("Failed to enqueue event action: {err}")
    }
}

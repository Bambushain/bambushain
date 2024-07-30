use crate::models::EventAction;
use crate::worker::publish;
use crate::Queue;

pub async fn enqueue_event(action: EventAction) {
    if let Err(err) = publish(Queue::Events, action).await {
        log::error!("Failed to enqueue event action: {err}")
    }
}

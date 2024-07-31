use crate::notifier::NotifierState;
use async_nats::{Client, Message};
use bamboo_common::backend::database::get_database;
use bamboo_common::backend::dbal;
use bamboo_common::backend::mq::{get_nats, FromMessage, NotificationError, Queue};
use bamboo_common::backend::notification::EventAction;
use futures_util::StreamExt;
use sea_orm::DatabaseConnection;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

pub async fn start_listening(
    notifier_state: NotifierState,
) -> Result<(JoinHandle<()>, CancellationToken), NotificationError> {
    log::info!("Start listening to events on nats");
    let db = get_database()
        .await
        .map_err(|err| NotificationError::new(err.to_string()))?;
    let nats_client = get_nats().await?;
    let stop_signal = CancellationToken::new();

    Ok((
        tokio::spawn(spawn_listen(
            db.clone(),
            nats_client.clone(),
            notifier_state.clone(),
            stop_signal.clone(),
        )),
        stop_signal.clone(),
    ))
}

pub(crate) async fn spawn_listen(
    db: DatabaseConnection,
    nats_client: Client,
    notifier_state: NotifierState,
    stop_signal: CancellationToken,
) {
    let subscriber = nats_client
        .subscribe(Queue::Events)
        .await
        .map_err(|err| NotificationError::new(err.to_string()));

    if let Ok(mut subscriber) = subscriber {
        loop {
            tokio::select! {
                _ = handle_message(subscriber.next().await, notifier_state.clone(), db.clone()) => { continue; }
                _ = stop_signal.cancelled() => {
                    log::info!("Gracefully shutting down event listener");
                    break;
                }
            }
        }
    }
}

async fn handle_message(
    message: Option<Message>,
    notifier_state: NotifierState,
    db: DatabaseConnection,
) {
    if let Some(message) = message {
        if let Ok(event_action) =
            EventAction::from_message(message).map_err(|err| log::error!("{err}"))
        {
            if let Ok(groves) = dbal::get_all_groves(&db).await {
                notifier_state.send_event(event_action, groves).await;
            }
        }
    }
}

mod event;

use crate::{IntoBytes, Queue};
pub use event::*;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct NotificationError {
    message: String,
}

impl Debug for NotificationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl Display for NotificationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl Error for NotificationError {}

impl NotificationError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

pub async fn get_nats() -> Result<async_nats::Client, NotificationError> {
    let client = async_nats::connect(
        std::env::var("NATS_SERVER").map_err(|err| NotificationError::new(err.to_string()))?,
    )
    .await
    .map_err(|err| NotificationError::new(err.to_string()))?;

    Ok(client)
}

pub(crate) async fn publish<P: IntoBytes>(
    queue: Queue,
    payload: P,
) -> Result<(), NotificationError> {
    let client = get_nats().await?;

    client
        .publish(queue, payload.into_bytes()?)
        .await
        .map_err(|err| NotificationError::new(err.to_string()))?;
    client
        .flush()
        .await
        .map_err(|err| NotificationError::new(err.to_string()))
}

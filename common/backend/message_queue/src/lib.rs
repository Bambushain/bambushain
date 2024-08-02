use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use async_nats::{jetstream, Message, Subject};
use async_nats::jetstream::{Context, stream};
use async_nats::jetstream::stream::Stream;
use async_nats::subject::ToSubject;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[macro_export]
macro_rules! impl_nats {
    ($type: ty) => {
        impl bamboo_common_backend_mq::IntoBytes for $type {
            fn into_bytes(
                self,
            ) -> Result<bytes::Bytes, bamboo_common_backend_mq::NotificationError> {
                let mut data = Vec::<u8>::new();

                ciborium::into_writer(&self, &mut data).map_err(|err| {
                    bamboo_common_backend_mq::NotificationError::new(err.to_string())
                })?;

                Ok(bytes::Bytes::copy_from_slice(data.as_slice()))
            }
        }

        impl bamboo_common_backend_mq::FromMessage<$type> for $type {
            fn from_message(
                message: async_nats::Message,
            ) -> Result<$type, bamboo_common_backend_mq::NotificationError> {
                ciborium::from_reader(message.payload.iter().as_slice()).map_err(|err| {
                    bamboo_common_backend_mq::NotificationError::new(err.to_string())
                })
            }

            fn from_jetstream_message(
                message: async_nats::jetstream::Message,
            ) -> Result<$type, bamboo_common_backend_mq::NotificationError> {
                ciborium::from_reader(message.payload.iter().as_slice()).map_err(|err| {
                    bamboo_common_backend_mq::NotificationError::new(err.to_string())
                })
            }
        }
    };
}

pub trait IntoBytes {
    fn into_bytes(self) -> Result<Bytes, NotificationError>;
}

pub trait FromMessage<T: Sized> {
    fn from_message(message: Message) -> Result<T, NotificationError>;
    fn from_jetstream_message(message: jetstream::Message) -> Result<T, NotificationError>;
}

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

pub async fn publish<P: IntoBytes>(queue: Queue, payload: P) -> Result<(), NotificationError> {
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

pub async fn get_jetstream() -> Result<Context, NotificationError> {
    let client = get_nats().await?;

    Ok(jetstream::new(client))
}

pub async fn get_once_stream() -> Result<Stream, NotificationError> {
    get_jetstream()
        .await?
        .create_stream(jetstream::stream::Config {
            name: "BAMBOO".to_string(),
            retention: stream::RetentionPolicy::WorkQueue,
            subjects: Queue::iter()
                .map(|queue| queue.to_subject().to_string())
                .collect::<Vec<String>>(),
            ..Default::default()
        })
        .await
        .map_err(|err| NotificationError::new(format!("Failed to create stream {err}")))
}

pub async fn publish_once<P: IntoBytes>(queue: Queue, payload: P) -> Result<(), NotificationError> {
    let jetstream = get_jetstream().await?;

    jetstream
        .publish(queue, payload.into_bytes()?)
        .await
        .map_err(|err| NotificationError::new(format!("Failed to publish {err}")))?
        .await
        .map_err(|err| NotificationError::new(format!("Failed to publish {err}")))
        .map(|_| ())
}

#[derive(EnumIter)]
pub enum Queue {
    Events,
    Mails,
}

impl Display for Queue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Queue::Events => f.write_str("bamboo.events"),
            Queue::Mails => f.write_str("bamboo.mails"),
        }
    }
}

impl ToSubject for Queue {
    fn to_subject(&self) -> Subject {
        Subject::from(self.to_string())
    }
}

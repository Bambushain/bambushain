use crate::impl_nats;
use async_nats::Message;
use bamboo_common_core::entities::*;
use bytes::Bytes;
use serde::{Deserialize, Serialize};

pub trait IntoBytes {
    fn into_bytes(self) -> Result<Bytes, NotificationError>;
}

pub trait FromMessage<T: Sized> {
    fn from_message(message: Message) -> Result<T, NotificationError>;
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum EventAction {
    #[serde(rename = "c")]
    Created(GroveEvent),
    #[serde(rename = "u")]
    Updated(GroveEvent),
    #[serde(rename = "d")]
    Deleted(GroveEvent),
}

impl_nats!(EventAction);

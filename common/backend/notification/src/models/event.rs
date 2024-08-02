use bamboo_common_core::entities::*;
use serde::{Deserialize, Serialize};
use bamboo_common_backend_mq::impl_nats;

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

pub(crate) mod macros;
mod models;
mod worker;

use async_nats::subject::ToSubject;
use async_nats::Subject;
pub use models::*;
use std::fmt::{Display, Formatter};
pub use worker::*;

pub enum Queue {
    Events,
}

impl Display for Queue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Queue::Events => f.write_str("bamboo.events"),
        }
    }
}

impl ToSubject for Queue {
    fn to_subject(&self) -> Subject {
        Subject::from(self.to_string())
    }
}

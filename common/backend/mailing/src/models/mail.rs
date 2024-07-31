use bamboo_common_backend_mq::impl_nats;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Mail {
    pub subject: String,
    pub to: String,
    pub body: String,
    pub reply_to: Option<String>,
}

impl Mail {
    pub fn new(
        subject: impl Into<String>,
        to: impl Into<String>,
        body: impl Into<String>,
        reply_to: Option<impl Into<String>>,
    ) -> Self {
        Mail {
            subject: subject.into(),
            to: to.into(),
            body: body.into(),
            reply_to: reply_to.map(|reply_to| reply_to.into()),
        }
    }
}

impl_nats!(Mail);

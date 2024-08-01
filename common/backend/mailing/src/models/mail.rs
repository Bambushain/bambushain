use bamboo_common_backend_mq::impl_nats;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Mail {
    pub subject: String,
    pub to: String,
    pub body: String,
    pub reply_to: Option<String>,
    pub templated: bool,
    pub action_label: Option<String>,
    pub action_link: Option<String>,
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
            templated: false,
            action_label: None,
            action_link: None,
        }
    }

    pub fn new_templated(
        subject: impl Into<String>,
        to: impl Into<String>,
        body: impl Into<String>,
        reply_to: Option<impl Into<String>>,
        action_label: impl Into<String>,
        action_link: impl Into<String>,
    ) -> Self {
        Mail {
            subject: subject.into(),
            to: to.into(),
            body: body.into(),
            reply_to: reply_to.map(|reply_to| reply_to.into()),
            templated: true,
            action_label: Some(action_label.into()),
            action_link: Some(action_link.into()),
        }
    }
}

impl_nats!(Mail);

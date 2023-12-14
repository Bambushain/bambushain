use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct SupportRequest {
    pub subject: String,
    pub message: String,
}

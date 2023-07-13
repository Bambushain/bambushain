use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct Mount {
    pub name: String,
}
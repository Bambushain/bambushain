use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct Kill {
    pub name: String,
}
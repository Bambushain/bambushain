use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct Kill {
    pub name: String,
}
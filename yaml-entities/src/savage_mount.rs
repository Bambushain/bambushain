use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct SavageMount {
    pub name: String,
}
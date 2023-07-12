use std::cmp::Ordering;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub struct Crafter {
    pub job: String,
    pub level: String,
}

impl PartialOrd<Self> for Crafter {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.job.to_lowercase().partial_cmp(&other.job.to_lowercase())
    }
}

impl Ord for Crafter {
    fn cmp(&self, other: &Self) -> Ordering {
        self.job.to_lowercase().cmp(&other.job.to_lowercase())
    }
}

#[derive(Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct UpdateCrafter {
    pub level: String,
}
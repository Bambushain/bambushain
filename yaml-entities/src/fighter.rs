use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Fighter {
    pub job: String,
    pub level: String,
    pub gear_score: String,
}

impl PartialOrd<Self> for Fighter {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.job.to_lowercase().partial_cmp(&other.job.to_lowercase())
    }
}

impl Ord for Fighter {
    fn cmp(&self, other: &Self) -> Ordering {
        self.job.to_lowercase().cmp(&other.job.to_lowercase())
    }
}

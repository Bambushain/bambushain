use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Fighter {
    pub job: String,
    pub level: String,
    pub gear_score: String
}

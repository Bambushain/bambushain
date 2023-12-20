use serde::{Deserialize, Serialize};

use bamboo_entities::prelude::*;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct Profile {
    pub user: WebUser,
}

impl From<WebUser> for Profile {
    fn from(value: WebUser) -> Self {
        Self { user: value }
    }
}

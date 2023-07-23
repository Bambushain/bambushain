use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use yew::AttrValue;

use crate::api::{get, SheefApiResult};
use crate::api::user::get_users;

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Hash, Default)]
pub struct CaseInsensitiveString(String);

impl Display for CaseInsensitiveString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_string().as_str())
    }
}

impl PartialOrd for CaseInsensitiveString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.to_lowercase().partial_cmp(&other.0.to_lowercase())
    }
}

impl Ord for CaseInsensitiveString {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.to_lowercase().cmp(&other.0.to_lowercase())
    }
}

impl From<AttrValue> for CaseInsensitiveString {
    fn from(value: AttrValue) -> Self {
        CaseInsensitiveString(value.to_string())
    }
}

#[derive(Clone, PartialEq, Default)]
pub struct BooleanTable {
    pub data: BTreeMap<CaseInsensitiveString, Vec<CaseInsensitiveString>>,
    pub keys: BTreeSet<CaseInsensitiveString>,
    pub users: BTreeSet<CaseInsensitiveString>,
}

pub async fn get_boolean_table(path: String) -> SheefApiResult<BooleanTable> {
    log::debug!("Get data from {}", path);
    let table_data = match get::<BTreeMap<CaseInsensitiveString, Vec<CaseInsensitiveString>>>(path).await {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    log::debug!("Get users");
    let users = match get_users().await {
        Ok(users) => users,
        Err(err) => return Err(err),
    };

    return Ok(BooleanTable {
        data: table_data.clone(),
        keys: table_data.clone().keys().cloned().collect::<BTreeSet<CaseInsensitiveString>>(),
        users: users.iter().map(|user| CaseInsensitiveString(user.username.clone())).collect::<BTreeSet<CaseInsensitiveString>>(),
    });
}
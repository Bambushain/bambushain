use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
use bamboo_macros::*;

use crate::user::WebUser;

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Login {
    pub email: String,
    pub password: String,
    pub two_factor_code: Option<String>,
}

impl Login {
    pub fn new(email: String, password: String, two_factor_code: Option<String>) -> Self {
        Self {
            email,
            password,
            two_factor_code,
        }
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestTwoFactor {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Responder))]
pub struct LoginResult {
    pub user: WebUser,
    pub token: String,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(not(target_arch = "wasm32"), derive(Responder))]
pub struct TwoFactorResult {
    pub user: WebUser,
    pub two_factor_code: Option<String>,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChangePassword {
    pub new_password: String,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMyPassword {
    pub old_password: String,
    pub new_password: String,
}

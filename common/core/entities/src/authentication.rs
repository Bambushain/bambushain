use serde::{Deserialize, Serialize};

use crate::User;
#[cfg(feature = "backend")]
use bamboo_common_backend_macros::*;

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Login {
    pub email: String,
    pub password: String,
    #[serde(default)]
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
#[cfg_attr(feature = "backend", derive(Responder))]
pub struct LoginResult {
    pub user: User,
    pub token: String,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "backend", derive(Responder))]
pub struct TwoFactorResult {
    pub user: User,
    pub requires_two_factor_code: bool,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMyPassword {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ForgotPassword {
    pub email: String,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResetPassword {
    pub email: String,
    pub token: String,
    pub password: String,
}

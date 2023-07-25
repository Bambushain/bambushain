use serde::{Deserialize, Serialize};

use crate::user::WebUser;

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
pub struct LoginResult {
    pub user: WebUser,
    pub token: String,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
pub struct ChangePassword {
    pub new_password: String,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
pub struct ChangeMyPassword {
    pub old_password: String,
    pub new_password: String,
}

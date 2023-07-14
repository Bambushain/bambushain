use serde::{Deserialize, Serialize};
use crate::User;

#[derive(Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct LoginResult {
    pub user: User,
    pub token: String,
}

#[derive(Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct ChangePassword {
    pub new_password: String,
}

#[derive(Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct ChangeMyPassword {
    pub old_password: String,
    pub new_password: String,
}

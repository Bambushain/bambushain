use std::cmp::Ordering;
use bcrypt::{BcryptError, hash};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub username: String,
    pub password: String,
    #[serde(default)]
    pub is_mod: bool,
    #[serde(rename = "mainGroup")]
    #[serde(default)]
    pub is_main_group: bool,
    #[serde(rename = "gearlevel")]
    #[serde(default)]
    pub gear_level: String,
    #[serde(default)]
    pub job: String,
    #[serde(default)]
    pub is_hidden: bool,
}

impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.username.to_lowercase().partial_cmp(&other.username.to_lowercase())
    }
}

impl Ord for User {
    fn cmp(&self, other: &Self) -> Ordering {
        self.username.to_lowercase().cmp(&other.username.to_lowercase())
    }
}

impl User {
    pub fn set_password(&mut self, plain_password: &String) -> Result<(), BcryptError> {
        let hashed = hash(plain_password.as_bytes(), 12);
        match hashed {
            Ok(hashed_password) => {
                self.password = hashed_password;
                Ok(())
            }
            Err(err) => Err(err)
        }
    }
}


#[derive(Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WebUser {
    pub username: String,
    pub is_mod: bool,
    pub is_main_group: bool,
    pub gear_level: String,
    pub job: String,
}
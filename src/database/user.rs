use std::env::join_paths;
use std::fs::remove_file;
use log::warn;
use bcrypt::{BcryptError, hash};
use serde::{Serialize, Deserialize};
use crate::database::{EmptyResult, persist_entity, read_entity, read_entity_dir, validate_database_dir};

#[derive(Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub username: String,
    pub password: String,
    pub is_mod: bool,
    #[serde(rename = "mainGroup")]
    pub is_main_group: bool,
    #[serde(rename = "gearlevel")]
    pub gear_level: String,
    pub job: String,
    pub is_hidden: bool,
}

impl User {
    pub fn set_password(&mut self, plain_password: &String) -> Result<(), BcryptError> {
        let hashed = hash(plain_password.as_bytes(), 12);
        match hashed {
            Ok(hashed_password) => {
                self.password = hashed_password;
                Ok(())
            }
            Err(err) => {
                Err(err)
            }
        }
    }
}

pub(crate) fn validate_user_dir() -> String {
    let path = join_paths(vec![validate_database_dir(), "user".to_string()]).expect("Paths should be able to join").into_string().expect("String should be available as core string");
    let result = std::fs::create_dir_all(path.as_str());
    if result.is_err() {
        panic!("Failed to create user database dir {}", result.err().unwrap());
    }

    path
}

pub fn create_user(username: &String, password: &String, is_mod: bool, is_main_group: bool, gear_level: &String, job: &String, is_hidden: bool) -> Option<User> {
    let mut user = User {
        username: username.to_string(),
        password: "".to_string(),
        is_main_group,
        is_mod,
        gear_level: gear_level.to_string(),
        job: job.to_string(),
        is_hidden,
    };
    if let Err(err) = user.set_password(password) {
        warn!("Failed to set user password {}", err);
        return None;
    }

    match persist_entity(validate_user_dir(), username, user) {
        Ok(user) => Some(user),
        _ => None
    }
}

pub fn delete_user(username: &String) -> EmptyResult {
    match remove_file(join_paths(vec![validate_user_dir(), format!("{}.yaml", username)]).expect("Paths should be able to join")) {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("Failed to delete user {}", err);
            Err(())
        }
    }
}

pub fn get_user(username: &String) -> Option<User> {
    read_entity(validate_user_dir(), username)
}

pub fn get_users() -> Option<impl Iterator<Item=User>> {
    read_entity_dir(validate_user_dir())
}

pub fn change_mod_status(username: &String, is_mod: bool) -> EmptyResult {
    let mut user = match get_user(username) {
        Some(user) => user,
        None => {
            warn!("User {} not found", username);
            return Err(());
        }
    };
    user.is_mod = is_mod;
    match persist_entity(validate_user_dir(), username, user) {
        Ok(_) => Ok(()),
        Err(_) => Err(())
    }
}

pub fn change_main_group(username: &String, is_main_group: bool) -> EmptyResult {
    let mut user = match get_user(username) {
        Some(user) => user,
        None => {
            warn!("User {} not found", username);
            return Err(());
        }
    };
    user.is_main_group = is_main_group;
    match persist_entity(validate_user_dir(), username, user) {
        Ok(_) => Ok(()),
        Err(_) => Err(())
    }
}

pub fn change_password(username: &String, password: &String) -> EmptyResult {
    let mut user = match get_user(username) {
        Some(user) => user,
        None => {
            warn!("User {} not found", username);
            return Err(());
        }
    };

    if let Err(err) = user.set_password(password) {
        warn!("Failed to set password for user {}: {}", username, err);
        return Err(());
    }

    match persist_entity(validate_user_dir(), username, user) {
        Ok(_) => Ok(()),
        Err(_) => Err(())
    }
}

pub fn update_me(username: &String, job: String, gear_level: String) -> EmptyResult {
    let mut user = match get_user(username) {
        Some(user) => user,
        None => {
            warn!("User {} not found", username);
            return Err(());
        }
    };

    user.gear_level = gear_level;
    user.job = job;

    match persist_entity(validate_user_dir(), username, user) {
        Ok(_) => Ok(()),
        Err(_) => Err(())
    }
}
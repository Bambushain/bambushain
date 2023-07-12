use std::fs::{remove_dir_all, remove_file};
use log::warn;
use sheef_entities::user::User;
use crate::{EmptyResult, persist_entity, read_entity, read_entity_dir, validate_database_dir};
use crate::token::get_user_token_dir;

pub(crate) fn validate_user_dir() -> String {
    let path = vec![validate_database_dir(), "user".to_string()].join("/");
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
    match remove_file(vec![validate_user_dir(), format!("{}.yaml", username)].join("/")) {
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
    Some(read_entity_dir::<User>(validate_user_dir()).unwrap().into_iter().filter(|user| !user.is_hidden))
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
        Ok(_) => {
            let _ = remove_dir_all(get_user_token_dir(username.to_string()).expect("User token dir cannot be empty"));
            Ok(())
        }
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
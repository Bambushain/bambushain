use std::fs::{create_dir_all, File, remove_dir_all, remove_file, rename};
use log::{error, warn};
use sheef_utils::sort_strings_insensitive;
use crate::{EmptyResult, validate_database_dir};
use crate::user::user_exists;

fn validate_kill_dir() -> String {
    let path = vec![validate_database_dir(), "kill".to_string()].join("/");
    let result = create_dir_all(path.as_str());
    if result.is_err() {
        panic!("Failed to create kill database dir {}", result.err().unwrap());
    }

    path
}

pub fn create_kill(kill: &String) -> EmptyResult {
    let path = vec![validate_kill_dir(), kill.to_string()].join("/");
    match create_dir_all(path.as_str()) {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("Failed to create kill dir ({}): {}", path, err);
            Err(())
        }
    }
}

pub fn delete_kill(kill: &String) -> EmptyResult {
    let path = vec![validate_kill_dir(), kill.to_string()].join("/");
    match remove_dir_all(path.as_str()) {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("Failed to delete kill dir ({}): {}", path, err);
            Err(())
        }
    }
}

pub fn update_kill(kill: &String, new_name: &String) -> EmptyResult {
    let old_path = vec![validate_kill_dir(), kill.to_string()].join("/");
    let new_path = vec![validate_kill_dir(), new_name.to_string()].join("/");
    match rename(old_path.as_str(), new_path.as_str()) {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("Failed to move kill dir ({} -> {}): {}", old_path, new_path, err);
            Err(())
        }
    }
}

pub fn activate_kill_for_user(kill: &String, username: &String) -> EmptyResult {
    let path = vec![validate_kill_dir(), kill.to_string(), username.to_string()].join("/");
    match File::create(path.as_str()) {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("Failed to activate kill {} for user {} ({}): {}", kill, username, path, err);
            Err(())
        }
    }
}

pub fn deactivate_kill_for_user(kill: &String, username: &String) -> EmptyResult {
    let path = vec![validate_kill_dir(), kill.to_string(), username.to_string()].join("/");
    match remove_file(path.as_str()) {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("Failed to deactivate kill {} for user {} ({}): {}", kill, username, path, err);
            Err(())
        }
    }
}

pub fn get_kills() -> Vec<String> {
    let mut kills = match std::fs::read_dir(validate_kill_dir()) {
        Ok(dir) => dir,
        Err(err) => {
            error!("Failed to load kill dirs {}", err);
            return vec![];
        }
    }.filter_map(|item| match item {
        Ok(entry) => match entry.path().is_dir() {
            true => Some(entry.file_name().into_string().expect("String should be available as core string")),
            false => None
        },
        Err(_) => None
    }).collect::<Vec<String>>();
    sort_strings_insensitive!(kills);
    kills
}

pub fn get_kills_for_user(username: &String) -> Option<Vec<String>> {
    let kills = get_kills();
    if !user_exists(username) {
        return None;
    }
    let mut for_user = kills.into_iter().filter_map(|kill| {
        match path_exists!(vec![validate_kill_dir(), kill.to_string(), username.to_string()].join("/")) {
            true => Some(kill),
            false => None
        }
    }).collect::<Vec<String>>();
    sort_strings_insensitive!(for_user);
    Some(for_user)
}

pub fn get_users_for_kill(kill: &String) -> Option<Vec<String>> {
    let path = vec![validate_kill_dir(), kill.to_string()].join("/");
    let mut users_for_kill = match std::fs::read_dir(path) {
        Ok(dir) => dir,
        Err(err) => {
            error!("Failed to load kill dirs {}", err);
            return None;
        }
    }.filter_map(|item| match item {
        Ok(entry) => match entry.path().is_file() {
            true => Some(entry.file_name().into_string().expect("String should be available as core string")),
            false => None
        },
        Err(_) => None
    }.filter(user_exists)).collect::<Vec<String>>();
    sort_strings_insensitive!(users_for_kill);
    Some(users_for_kill)
}

pub fn kill_exists(kill: &String) -> bool {
    path_exists!(vec![validate_kill_dir(), kill.to_string()].join("/"))
}

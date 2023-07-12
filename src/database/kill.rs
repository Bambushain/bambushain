use std::env::join_paths;
use std::fs::{create_dir_all, File, metadata, remove_dir_all, remove_file, rename};
use log::{error, warn};
use crate::database::{EmptyResult, validate_database_dir};

fn validate_kill_dir() -> String {
    let path = join_paths(vec![validate_database_dir(), "kill".to_string()]).expect("Paths should be able to join").into_string().expect("String should be available as core string");
    let result = create_dir_all(path.as_str());
    if result.is_err() {
        panic!("Failed to create kill database dir {}", result.err().unwrap());
    }

    path
}

pub fn create_kill(kill: &String) -> EmptyResult {
    let path = join_paths(vec![validate_kill_dir(), kill.to_string()]).expect("Paths should be able to join").into_string().expect("String should be available as core string");
    match create_dir_all(path.as_str()) {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("Failed to create kill dir ({}): {}", path, err);
            Err(())
        }
    }
}

pub fn delete_kill(kill: &String) -> EmptyResult {
    let path = join_paths(vec![validate_kill_dir(), kill.to_string()]).expect("Paths should be able to join").into_string().expect("String should be available as core string");
    match remove_dir_all(path.as_str()) {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("Failed to delete kill dir ({}): {}", path, err);
            Err(())
        }
    }
}

pub fn update_kill(kill: &String, new_name: &String) -> EmptyResult {
    let old_path = join_paths(vec![validate_kill_dir(), kill.to_string()]).expect("Paths should be able to join").into_string().expect("String should be available as core string");
    let new_path = join_paths(vec![validate_kill_dir(), new_name.to_string()]).expect("Paths should be able to join").into_string().expect("String should be available as core string");
    match rename(old_path.as_str(), new_path.as_str()) {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("Failed to move kill dir ({} -> {}): {}", old_path, new_path, err);
            Err(())
        }
    }
}

pub fn activate_kill_for_user(kill: &String, username: &String) -> EmptyResult {
    let path = join_paths(vec![validate_kill_dir(), kill.to_string(), username.to_string()]).expect("Paths should be able to join").into_string().expect("String should be available as core string");
    match File::create(path.as_str()) {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("Failed to activate kill {} for user {} ({}): {}", kill, username, path, err);
            Err(())
        }
    }
}

pub fn deactivate_kill_for_user(kill: &String, username: &String) -> EmptyResult {
    let path = join_paths(vec![validate_kill_dir(), kill.to_string(), username.to_string()]).expect("Paths should be able to join").into_string().expect("String should be available as core string");
    match remove_file(path.as_str()) {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("Failed to deactivate kill {} for user {} ({}): {}", kill, username, path, err);
            Err(())
        }
    }
}

pub fn get_kills() -> Option<impl Iterator<Item=String>> {
    Some(match std::fs::read_dir(validate_kill_dir()) {
        Ok(dir) => dir,
        Err(err) => {
            error!("Failed to load kill dirs {}", err);
            return None;
        }
    }.filter_map(|item| match item {
        Ok(entry) => match entry.path().is_dir() {
            true => Some(entry.file_name().into_string().expect("String should be available as core string")),
            false => None
        },
        Err(_) => None
    }))
}

pub fn get_kills_for_user(username: &String) -> Option<impl Iterator<Item=String> + '_> {
    let kills = match get_kills() {
        Some(kills) => kills,
        None => return None
    };
    Some(kills.filter_map(|kill| {
        let kill_path = join_paths(vec![validate_kill_dir(), kill.to_string(), username.to_string()]).expect("Paths should be able to join");
        let has_user = match metadata(kill_path) {
            Ok(res) => res.is_file(),
            Err(_) => false,
        };
        match has_user {
            true => Some(kill),
            false => None
        }
    }))
}

pub fn get_users_for_kill(kill: &String) -> Option<impl Iterator<Item=String>> {
    let path = join_paths(vec![validate_kill_dir(), kill.to_string()]).expect("Paths should be able to join").into_string().expect("String should be available as core string");
    Some(match std::fs::read_dir(path) {
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
    }))
}
use std::env::join_paths;
use std::fs::{create_dir_all, File, metadata, remove_dir_all, remove_file, rename};
use log::{error, warn};
use crate::database::{EmptyResult, validate_database_dir};

fn validate_mount_dir() -> String {
    let path = join_paths(vec![validate_database_dir(), "mount".to_string()]).expect("Paths should be able to join").into_string().expect("String should be available as core string");
    let result = create_dir_all(path.as_str());
    if result.is_err() {
        panic!("Failed to create mount database dir {}", result.err().unwrap());
    }

    path
}

pub fn create_mount(mount: &String) -> EmptyResult {
    let path = join_paths(vec![validate_mount_dir(), mount.to_string()]).expect("Paths should be able to join").into_string().expect("String should be available as core string");
    match create_dir_all(path.as_str()) {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("Failed to create mount dir ({}): {}", path, err);
            Err(())
        }
    }
}

pub fn delete_mount(mount: &String) -> EmptyResult {
    let path = join_paths(vec![validate_mount_dir(), mount.to_string()]).expect("Paths should be able to join").into_string().expect("String should be available as core string");
    match remove_dir_all(path.as_str()) {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("Failed to delete mount dir ({}): {}", path, err);
            Err(())
        }
    }
}

pub fn update_mount(mount: &String, new_name: &String) -> EmptyResult {
    let old_path = join_paths(vec![validate_mount_dir(), mount.to_string()]).expect("Paths should be able to join").into_string().expect("String should be available as core string");
    let new_path = join_paths(vec![validate_mount_dir(), new_name.to_string()]).expect("Paths should be able to join").into_string().expect("String should be available as core string");
    match rename(old_path.as_str(), new_path.as_str()) {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("Failed to move mount dir ({} -> {}): {}", old_path, new_path, err);
            Err(())
        }
    }
}

pub fn activate_mount_for_user(mount: &String, username: &String) -> EmptyResult {
    let path = join_paths(vec![validate_mount_dir(), mount.to_string(), username.to_string()]).expect("Paths should be able to join").into_string().expect("String should be available as core string");
    match File::create(path.as_str()) {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("Failed to activate mount {} for user {} ({}): {}", mount, username, path, err);
            Err(())
        }
    }
}

pub fn deactivate_mount_for_user(mount: &String, username: &String) -> EmptyResult {
    let path = join_paths(vec![validate_mount_dir(), mount.to_string(), username.to_string()]).expect("Paths should be able to join").into_string().expect("String should be available as core string");
    match remove_file(path.as_str()) {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("Failed to deactivate mount {} for user {} ({}): {}", mount, username, path, err);
            Err(())
        }
    }
}

pub fn get_mounts() -> Option<impl Iterator<Item=String>> {
    Some(match std::fs::read_dir(validate_mount_dir()) {
        Ok(dir) => dir,
        Err(err) => {
            error!("Failed to load mount dirs {}", err);
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

pub fn get_mounts_for_user(username: &String) -> Option<impl Iterator<Item=String> + '_> {
    let mounts = match get_mounts() {
        Some(mounts) => mounts,
        None => return None
    };
    Some(mounts.filter_map(|mount| {
        let mount_path = join_paths(vec![validate_mount_dir(), mount.to_string(), username.to_string()]).expect("Paths should be able to join");
        let has_user = match metadata(mount_path) {
            Ok(res) => res.is_file(),
            Err(_) => false,
        };
        match has_user {
            true => Some(mount),
            false => None
        }
    }))
}

pub fn get_users_for_mount(mount: &String) -> Option<impl Iterator<Item=String>> {
    let path = join_paths(vec![validate_mount_dir(), mount.to_string()]).expect("Paths should be able to join").into_string().expect("String should be available as core string");
    Some(match std::fs::read_dir(path) {
        Ok(dir) => dir,
        Err(err) => {
            error!("Failed to load mount dirs {}", err);
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
use std::fs::{create_dir_all, File, metadata, remove_dir_all, remove_file, rename};
use log::{error, warn};
use crate::{EmptyResult, validate_database_dir};

fn validate_savage_mount_dir() -> String {
    let path = vec![validate_database_dir(), "savageMount".to_string()].join("/");
    let result = create_dir_all(path.as_str());
    if result.is_err() {
        panic!("Failed to create savage mount database dir {}", result.err().unwrap());
    }

    path
}

pub fn create_savage_mount(savage_mount: &String) -> EmptyResult {
    let path = vec![validate_savage_mount_dir(), savage_mount.to_string()].join("/");
    match create_dir_all(path.as_str()) {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("Failed to create savage mount dir ({}): {}", path, err);
            Err(())
        }
    }
}

pub fn delete_savage_mount(savage_mount: &String) -> EmptyResult {
    let path = vec![validate_savage_mount_dir(), savage_mount.to_string()].join("/");
    match remove_dir_all(path.as_str()) {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("Failed to delete savage mount dir ({}): {}", path, err);
            Err(())
        }
    }
}

pub fn update_savage_mount(savage_mount: &String, new_name: &String) -> EmptyResult {
    let old_path = vec![validate_savage_mount_dir(), savage_mount.to_string()].join("/");
    let new_path = vec![validate_savage_mount_dir(), new_name.to_string()].join("/");
    match rename(old_path.as_str(), new_path.as_str()) {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("Failed to move savage mount dir ({} -> {}): {}", old_path, new_path, err);
            Err(())
        }
    }
}

pub fn activate_savage_mount_for_user(savage_mount: &String, username: &String) -> EmptyResult {
    let path = vec![validate_savage_mount_dir(), savage_mount.to_string(), username.to_string()].join("/");
    match File::create(path.as_str()) {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("Failed to activate savage mount {} for user {} ({}): {}", savage_mount, username, path, err);
            Err(())
        }
    }
}

pub fn deactivate_savage_mount_for_user(savage_mount: &String, username: &String) -> EmptyResult {
    let path = vec![validate_savage_mount_dir(), savage_mount.to_string(), username.to_string()].join("/");
    match remove_file(path.as_str()) {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("Failed to deactivate savage mount {} for user {} ({}): {}", savage_mount, username, path, err);
            Err(())
        }
    }
}

pub fn get_savage_mounts() -> Option<impl Iterator<Item=String>> {
    Some(match std::fs::read_dir(validate_savage_mount_dir()) {
        Ok(dir) => dir,
        Err(err) => {
            error!("Failed to load savage mount dirs {}", err);
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

pub fn get_savage_mounts_for_user(username: &String) -> Option<impl Iterator<Item=String> + '_> {
    let savage_mounts = match get_savage_mounts() {
        Some(savage_mounts) => savage_mounts,
        None => return None
    };
    Some(savage_mounts.filter_map(|savage_mount| {
        let savage_mount_path = vec![validate_savage_mount_dir(), savage_mount.to_string(), username.to_string()].join("/");
        let has_user = match metadata(savage_mount_path) {
            Ok(res) => res.is_file(),
            Err(_) => false,
        };
        match has_user {
            true => Some(savage_mount),
            false => None
        }
    }))
}

pub fn get_users_for_savage_mount(savage_mount: &String) -> Option<impl Iterator<Item=String>> {
    let path = vec![validate_savage_mount_dir(), savage_mount.to_string()].join("/");
    Some(match std::fs::read_dir(path) {
        Ok(dir) => dir,
        Err(err) => {
            error!("Failed to load savage mount dirs {}", err);
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
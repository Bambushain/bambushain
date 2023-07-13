use std::fs::{create_dir_all, File, remove_dir_all, remove_file, rename};
use log::{error, warn};
use sheef_utils::sort_strings_insensitive;
use crate::{EmptyResult, validate_database_dir};
use crate::user::user_exists;

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

pub fn get_savage_mounts() -> Vec<String> {
    match std::fs::read_dir(validate_savage_mount_dir()) {
        Ok(dir) => dir,
        Err(err) => {
            error!("Failed to load savage mount dirs {}", err);
            return vec![];
        }
    }.filter_map(|item| match item {
        Ok(entry) => match entry.path().is_dir() {
            true => Some(entry.file_name().into_string().expect("String should be available as core string")),
            false => None
        },
        Err(_) => None
    }).collect::<Vec<String>>()
}

pub fn get_savage_mounts_for_user(username: &String) -> Option<Vec<String>> {
    let savage_mounts = get_savage_mounts();
    if !user_exists(username) {
        return None;
    }
    let mut for_user = savage_mounts.into_iter().filter_map(|savage_mount| {
        match path_exists!(vec![validate_savage_mount_dir(), savage_mount.to_string(), username.to_string()].join("/")) {
            true => Some(savage_mount),
            false => None
        }
    }).collect::<Vec<String>>();
    sort_strings_insensitive!(for_user);
    Some(for_user)
}

pub fn get_users_for_savage_mount(savage_mount: &String) -> Option<Vec<String>> {
    let path = vec![validate_savage_mount_dir(), savage_mount.to_string()].join("/");
    let mut users_for_savage_mount = match std::fs::read_dir(path) {
        Ok(dir) => dir,
        Err(err) => {
            error!("Failed to load savage_mount dirs {}", err);
            return None;
        }
    }.filter_map(|item| match item {
        Ok(entry) => match entry.path().is_file() {
            true => Some(entry.file_name().into_string().expect("String should be available as core string")),
            false => None
        },
        Err(_) => None
    }.filter(user_exists)).collect::<Vec<String>>();
    sort_strings_insensitive!(users_for_savage_mount);
    Some(users_for_savage_mount)
}

pub fn savage_mount_exists(savage_mount: &String) -> bool {
    path_exists!(vec![validate_savage_mount_dir(), savage_mount.to_string()].join("/"))
}

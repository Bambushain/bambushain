use std::fs::{create_dir_all, File, remove_dir_all, remove_file, rename};
use log::{error, warn};
use sheef_utils::sort_strings_insensitive;
use crate::{EmptyResult, validate_database_dir};
use crate::user::user_exists;

fn validate_mount_dir() -> String {
    let path = vec![validate_database_dir(), "mount".to_string()].join("/");
    let result = create_dir_all(path.as_str());
    if result.is_err() {
        panic!("Failed to create mount database dir {}", result.err().unwrap());
    }

    path
}

pub fn create_mount(mount: &String) -> EmptyResult {
    let path = vec![validate_mount_dir(), mount.to_string()].join("/");
    match create_dir_all(path.as_str()) {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("Failed to create mount dir ({}): {}", path, err);
            Err(())
        }
    }
}

pub fn delete_mount(mount: &String) -> EmptyResult {
    let path = vec![validate_mount_dir(), mount.to_string()].join("/");
    match remove_dir_all(path.as_str()) {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("Failed to delete mount dir ({}): {}", path, err);
            Err(())
        }
    }
}

pub fn update_mount(mount: &String, new_name: &String) -> EmptyResult {
    let old_path = vec![validate_mount_dir(), mount.to_string()].join("/");
    let new_path = vec![validate_mount_dir(), new_name.to_string()].join("/");
    match rename(old_path.as_str(), new_path.as_str()) {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("Failed to move mount dir ({} -> {}): {}", old_path, new_path, err);
            Err(())
        }
    }
}

pub fn activate_mount_for_user(mount: &String, username: &String) -> EmptyResult {
    let path = vec![validate_mount_dir(), mount.to_string(), username.to_string()].join("/");
    match File::create(path.as_str()) {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("Failed to activate mount {} for user {} ({}): {}", mount, username, path, err);
            Err(())
        }
    }
}

pub fn deactivate_mount_for_user(mount: &String, username: &String) -> EmptyResult {
    let path = vec![validate_mount_dir(), mount.to_string(), username.to_string()].join("/");
    match remove_file(path.as_str()) {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("Failed to deactivate mount {} for user {} ({}): {}", mount, username, path, err);
            Err(())
        }
    }
}

pub fn get_mounts() -> Vec<String> {
    match std::fs::read_dir(validate_mount_dir()) {
        Ok(dir) => dir,
        Err(err) => {
            error!("Failed to load mount dirs {}", err);
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

pub fn get_mounts_for_user(username: &String) -> Option<Vec<String>> {
    let mounts = get_mounts();
    if !user_exists(username) {
        return None;
    }
    let mut for_user = mounts.into_iter().filter_map(|mount| {
        match path_exists!(vec![validate_mount_dir(), mount.to_string(), username.to_string()].join("/")) {
            true => Some(mount),
            false => None
        }
    }).collect::<Vec<String>>();
    sort_strings_insensitive!(for_user);
    Some(for_user)
}

pub fn get_users_for_mount(mount: &String) -> Option<Vec<String>> {
    let path = vec![validate_mount_dir(), mount.to_string()].join("/");
    let mut users_for_mount = match std::fs::read_dir(path) {
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
    }.filter(user_exists)).collect::<Vec<String>>();
    sort_strings_insensitive!(users_for_mount);
    Some(users_for_mount)
}

pub fn mount_exists(mount: &String) -> bool {
    path_exists!(vec![validate_mount_dir(), mount.to_string()].join("/"))
}

use std::env::{current_dir, join_paths};
use std::fs::File;
use log::{error, warn};
use serde::{Deserialize, Serialize};


pub(crate) fn validate_database_dir() -> String {
    let path = join_paths(vec![current_dir().expect("Current dir is not available").into_os_string().to_str().unwrap(), "data"]).expect("Paths should be able to join").into_string().expect("String should be available as core string");
    let result = std::fs::create_dir_all(path.as_str());
    if result.is_err() {
        panic!("Failed to create database dir {}", result.err().unwrap());
    }

    path
}

pub(crate) fn persist_entity<T: Serialize>(base_path: impl Into<String>, filename: impl Into<String>, entity: T) -> Result<T, ()> {
    let path = join_paths(vec![base_path.into(), format!("{}.yaml", filename.into())]).expect("Paths should be able to join").into_string().expect("String should be available as core string");
    let file = match File::create(path.as_str()) {
        Ok(file) => file,
        Err(err) => {
            warn!("Failed to create file {}: {}", path, err);
            return Err(());
        }
    };

    match serde_yaml::to_writer(file, &entity) {
        Ok(_) => Ok(entity),
        Err(err) => {
            warn!("Failed to serialize entity ({}): {}", path, err);
            Err(())
        }
    }
}

pub(crate) fn read_entity<T: for<'a> Deserialize<'a>>(base_path: impl Into<String>, filename: impl Into<String>) -> Option<T> {
    let path = join_paths(vec![base_path.into(), format!("{}.yaml", filename.into())]).expect("Paths should be able to join").into_string().expect("String should be available as core string");
    let file = match File::open(path.as_str()) {
        Ok(file) => file,
        Err(err) => {
            warn!("Failed to read entity ({}): {}", path, err);
            return None;
        }
    };

    serde_yaml::from_reader(file).ok()
}

pub(crate) fn read_entity_dir<T: for<'a> Deserialize<'a>>(path: String) -> Option<impl Iterator<Item=T>> {
    Some(match std::fs::read_dir(path) {
        Ok(dir) => dir,
        Err(err) => {
            error!("Failed to load entity files {}",err);
            return None;
        }
    }.filter_map(|item| match item {
        Ok(entry) => {
            if !entry.path().is_file() || !entry.path().ends_with(".yaml") {
                return None;
            }
            let data = match std::fs::read_to_string(entry.path()) {
                Ok(data) => data,
                Err(err) => {
                    error!("Failed to load entity path ({}): {}", entry.path().to_str().expect("str conversion should be possible"), err);
                    return None;
                }
            };
            match serde_yaml::from_str::<T>(data.as_str()) {
                Ok(user) => Some(user),
                Err(err) => {
                    error!("Failed to deserialize entity ({}): {}", entry.path().to_str().expect("str conversion should be possible"), err);
                    None
                }
            }
        }
        Err(err) => {
            error!("Invalid DirEntry {}", err);
            None
        }
    }))
}

pub type EmptyResult = Result<(), ()>;

pub mod user;
pub mod token;
pub mod crafter;
pub mod fighter;
pub mod kill;
pub mod mount;
pub mod savage_mount;
pub mod event;
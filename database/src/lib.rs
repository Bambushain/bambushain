use std::env::current_dir;
use std::fs::File;
use log::{error, warn};
use serde::{Deserialize, Serialize};

macro_rules! path_exists {
    ($path:expr) => {
        std::fs::metadata($path).is_ok()
    };
}

pub(crate) fn validate_database_dir() -> String {
    let path = vec![current_dir().expect("Current dir is not available").into_os_string().to_str().unwrap(), "data"].join("/");
    let result = std::fs::create_dir_all(path.as_str());
    if result.is_err() {
        panic!("Failed to create database dir {}", result.err().unwrap());
    }

    path
}

pub(crate) fn persist_entity<T: Serialize>(base_path: impl Into<String>, filename: impl Into<String>, entity: T) -> Result<T, ()> {
    let path = vec![base_path.into(), format!("{}.yaml", filename.into())].join("/");
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
    let path = vec![base_path.into(), format!("{}.yaml", filename.into())].join("/");
    let file = match File::open(path.as_str()) {
        Ok(file) => file,
        Err(err) => {
            warn!("Failed to read entity ({}): {}", path, err);
            return None;
        }
    };

    let res = serde_yaml::from_reader::<File, T>(file);
    match res {
        Ok(data) => Some(data),
        Err(err) => {
            warn!("Failed to deserialize entity ({}): {}", path, err);
            None
        }
    }
}

pub(crate) fn read_entity_dir<T: for<'a> Deserialize<'a>>(path: String) -> Option<Vec<T>> where T: Ord, T: PartialOrd, T: Eq, T: PartialEq {
    let mut data = match std::fs::read_dir(path) {
        Ok(dir) => dir,
        Err(err) => {
            error!("Failed to load entity files {}", err);
            return None;
        }
    }.filter_map(|item| match item {
        Ok(entry) => {
            let path = entry.path();
            let path_as_str = path.to_str().expect("str conversion should be possible");
            if !path.is_file() {
                None
            } else if let Ok(data) = std::fs::read_to_string(path_as_str) {
                match serde_yaml::from_str::<T>(data.as_str()) {
                    Ok(entity) => Some(entity),
                    Err(err) => {
                        error!("Failed to deserialize entity ({}): {}", path_as_str, err);
                        None
                    }
                }
            } else {
                error!("Failed to load entity path ({})", path_as_str);
                None
            }
        }
        Err(err) => {
            error!("Invalid DirEntry {}", err);
            None
        }
    }).collect::<Vec<T>>();
    data.sort();
    Some(data)
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
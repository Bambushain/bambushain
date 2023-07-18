use std::env::current_dir;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;

macro_rules! path_exists {
    ($path:expr) => {
        tokio::fs::metadata($path).await.is_ok()
    };
}

fn get_db_dir() -> String {
    vec![current_dir().expect("Current dir is not available").into_os_string().to_str().unwrap(), "data"].join("/")
}

pub(crate) async fn validate_database_dir() -> String {
    let path = get_db_dir();
    let result = tokio::fs::create_dir_all(path.as_str()).await;
    if result.is_err() {
        panic!("Failed to create database dir {}", result.err().unwrap());
    }

    path
}

pub(crate) fn validate_database_dir_sync() -> String {
    let path = get_db_dir();
    let result = std::fs::create_dir_all(path.as_str());
    if result.is_err() {
        panic!("Failed to create database dir {}", result.err().unwrap());
    }

    path
}

pub(crate) async fn persist_entity<T: Serialize>(base_path: impl Into<String>, filename: impl Into<String>, entity: T) -> Result<T, ()> where T: Clone {
    let path = vec![base_path.into(), format!("{}.yaml", filename.into())].join("/");
    let mut file = match tokio::fs::File::create(path.as_str()).await {
        Ok(file) => file,
        Err(err) => {
            log::warn!("Failed to create file {}: {}", path, err);
            return Err(());
        }
    };

    let io_res = match serde_yaml::to_string(&entity) {
        Ok(yaml) => file.write_all(yaml.as_bytes()).await,
        Err(err) => {
            log::warn!("Failed to serialize entity: {}", err);
            return Err(());
        }
    };

    match io_res {
        Ok(_) => Ok(entity),
        Err(err) => {
            log::warn!("Failed to write entity ({}): {}", path, err);
            Err(())
        }
    }
}

pub(crate) async fn read_entity<T: for<'a> Deserialize<'a>>(base_path: impl Into<String>, filename: impl Into<String>) -> Option<T> {
    let path = vec![base_path.into(), format!("{}.yaml", filename.into())].join("/");
    let file_data = match tokio::fs::read_to_string(path.as_str()).await {
        Err(err) => {
            log::warn!("Failed to read file from {path}: {err}");
            return None;
        }
        Ok(data) => data
    };

    let res = serde_yaml::from_slice(file_data.as_bytes());
    match res {
        Ok(data) => Some(data),
        Err(err) => {
            log::warn!("Failed to deserialize entity ({}): {}", path, err);
            None
        }
    }
}

pub(crate) fn read_entity_sync<T: for<'a> Deserialize<'a>>(base_path: impl Into<String>, filename: impl Into<String>) -> Option<T> {
    let path = vec![base_path.into(), format!("{}.yaml", filename.into())].join("/");
    let file_data = match std::fs::read_to_string(path.as_str()) {
        Err(err) => {
            log::warn!("Failed to read file from {path}: {err}");
            return None;
        }
        Ok(data) => data
    };

    let res = serde_yaml::from_slice(file_data.as_bytes());
    match res {
        Ok(data) => Some(data),
        Err(err) => {
            log::warn!("Failed to deserialize entity ({}): {}", path, err);
            None
        }
    }
}

pub(crate) async fn read_entity_dir<T: for<'a> Deserialize<'a>>(path: String) -> Option<Vec<T>> where T: Ord, T: PartialOrd, T: Eq, T: PartialEq {
    let read_dir = match tokio::fs::read_dir(path).await {
        Ok(dir) => dir,
        Err(err) => {
            log::error!("Failed to load entity files {}", err);
            return None;
        }
    };

    let mut result_data = vec![];

    let mut tokio_data = tokio_stream::wrappers::ReadDirStream::new(read_dir);
    while let Some(item) = tokio_data.next().await {
        match item {
            Ok(entry) => {
                let path = entry.path();
                let path_as_str = path.to_str().expect("str conversion should be possible");
                if !path.is_file() {
                    continue;
                } else if let Ok(data) = tokio::fs::read_to_string(path_as_str).await {
                    match serde_yaml::from_str::<T>(data.as_str()) {
                        Ok(entity) => result_data.push(entity),
                        Err(err) => {
                            log::error!("Failed to deserialize entity ({}): {}", path_as_str, err);
                        }
                    }
                } else {
                    log::error!("Failed to load entity path ({})", path_as_str);
                }
            }
            Err(err) => {
                log::error!("Invalid DirEntry {}", err);
            }
        }
    };

    result_data.sort();

    Some(result_data)
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
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;

use sheef_entities::{sheef_io_error, sheef_serialization_error, SheefError};

macro_rules! path_exists {
    ($path:expr) => {
        tokio::fs::metadata($path).await.is_ok()
    };
}

macro_rules! map_err {
    ($entity_type:expr, $result:expr) => {
        $entity_type.map_err(|mut err| {
            err.entity_type = $result.to_string();
            err
        })
    };
}

fn get_db_dir() -> String {
    vec![sheef_utils::get_database_base_dir(), "data".to_string()].join("/")
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

pub(crate) async fn persist_entity<T: Serialize>(base_path: impl Into<String>, filename: impl Into<String>, entity: T) -> SheefResult<T> where T: Clone {
    let path = vec![base_path.into(), format!("{}.yaml", filename.into())].join("/");
    let mut file = match tokio::fs::File::create(path.as_str()).await {
        Ok(file) => file,
        Err(err) => {
            log::warn!("Failed to create file {}: {}", path, err);
            return Err(sheef_io_error!("", "Failed to create file"));
        }
    };

    let io_res = match serde_yaml::to_string(&entity) {
        Ok(yaml) => file.write_all(yaml.as_bytes()).await,
        Err(err) => {
            log::warn!("Failed to serialize entity: {}", err);
            return Err(sheef_serialization_error!("", "Failed to serialize entity"));
        }
    };

    match io_res {
        Ok(_) => Ok(entity),
        Err(err) => {
            log::warn!("Failed to write entity ({}): {}", path, err);
            Err(sheef_io_error!("", "Failed to write entity to file"))
        }
    }
}

pub(crate) async fn read_entity<T: for<'a> Deserialize<'a>>(base_path: impl Into<String>, filename: impl Into<String>) -> SheefResult<T> {
    let path = vec![base_path.into(), format!("{}.yaml", filename.into())].join("/");
    let file_data = match tokio::fs::read_to_string(path.as_str()).await {
        Err(err) => {
            log::warn!("Failed to read file from {path}: {err}");
            return Err(sheef_io_error!("", "Failed to read file"));
        }
        Ok(data) => data
    };

    let res = serde_yaml::from_slice(file_data.as_bytes());
    match res {
        Ok(data) => Ok(data),
        Err(err) => {
            log::warn!("Failed to deserialize entity ({}): {}", path, err);
            Err(sheef_serialization_error!("", "Failed to deserialize entity"))
        }
    }
}

pub(crate) fn read_entity_sync<T: for<'a> Deserialize<'a>>(base_path: impl Into<String>, filename: impl Into<String>) -> SheefResult<T> {
    let path = vec![base_path.into(), format!("{}.yaml", filename.into())].join("/");
    let file_data = match std::fs::read_to_string(path.as_str()) {
        Err(err) => {
            log::warn!("Failed to read file from {path}: {err}");
            return Err(sheef_io_error!("", "Failed to read file"));
        }
        Ok(data) => data
    };

    let res = serde_yaml::from_slice(file_data.as_bytes());
    match res {
        Ok(data) => Ok(data),
        Err(err) => {
            log::warn!("Failed to deserialize entity ({}): {}", path, err);
            Err(sheef_serialization_error!("", "Failed to deserialize entity"))
        }
    }
}

pub(crate) async fn read_entity_dir<T: for<'a> Deserialize<'a>>(path: String) -> SheefResult<Vec<T>> where T: Ord, T: PartialOrd, T: Eq, T: PartialEq {
    let read_dir = match tokio::fs::read_dir(path).await {
        Ok(dir) => dir,
        Err(err) => {
            log::error!("Failed to load entity files {}", err);
            return Err(sheef_io_error!("", "Failed to load entity files"));
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

    Ok(result_data)
}

pub type SheefErrorResult = Result<(), SheefError>;

pub type SheefResult<T> = Result<T, SheefError>;

pub mod user;
pub mod token;
pub mod crafter;
pub mod fighter;
pub mod kill;
pub mod mount;
pub mod savage_mount;
pub mod event;
use tokio_stream::StreamExt;
use sheef_utils::sort_strings_insensitive;
use crate::{EmptyResult, validate_database_dir};
use crate::user::user_exists;

async fn validate_mount_dir() -> String {
    let path = vec![validate_database_dir().await, "savageMount".to_string()].join("/");
    let result = tokio::fs::create_dir_all(path.as_str()).await;
    if result.is_err() {
        panic!("Failed to create mount database dir {}", result.err().unwrap());
    }

    path
}

pub async fn create_mount(mount: &String) -> EmptyResult {
    let path = vec![validate_mount_dir().await, mount.to_string()].join("/");
    match tokio::fs::create_dir_all(path.as_str()).await {
        Ok(_) => Ok(()),
        Err(err) => {
            log::warn!("Failed to create mount dir ({}): {}", path, err);
            Err(())
        }
    }
}

pub async fn delete_mount(mount: &String) -> EmptyResult {
    let path = vec![validate_mount_dir().await, mount.to_string()].join("/");
    match tokio::fs::remove_dir_all(path.as_str()).await {
        Ok(_) => Ok(()),
        Err(err) => {
            log::warn!("Failed to delete mount dir ({}): {}", path, err);
            Err(())
        }
    }
}

pub async fn update_mount(mount: &String, new_name: &String) -> EmptyResult {
    let old_path = vec![validate_mount_dir().await, mount.to_string()].join("/");
    let new_path = vec![validate_mount_dir().await, new_name.to_string()].join("/");
    match tokio::fs::rename(old_path.as_str(), new_path.as_str()).await {
        Ok(_) => Ok(()),
        Err(err) => {
            log::warn!("Failed to move mount dir ({} -> {}): {}", old_path, new_path, err);
            Err(())
        }
    }
}

pub async fn activate_mount_for_user(mount: &String, username: &String) -> EmptyResult {
    let path = vec![validate_mount_dir().await, mount.to_string(), username.to_string()].join("/");
    match tokio::fs::File::create(path.as_str()).await {
        Ok(_) => Ok(()),
        Err(err) => {
            log::warn!("Failed to activate mount {} for user {} ({}): {}", mount, username, path, err);
            Err(())
        }
    }
}

pub async fn deactivate_mount_for_user(mount: &String, username: &String) -> EmptyResult {
    let path = vec![validate_mount_dir().await, mount.to_string(), username.to_string()].join("/");
    match tokio::fs::remove_file(path.as_str()).await {
        Ok(_) => Ok(()),
        Err(err) => {
            log::warn!("Failed to deactivate mount {} for user {} ({}): {}", mount, username, path, err);
            Err(())
        }
    }
}

pub async fn get_mounts() -> Vec<String> {
    let read_dir = match tokio::fs::read_dir(validate_mount_dir().await).await {
        Ok(dir) => dir,
        Err(err) => {
            log::error!("Failed to load mount dirs {}", err);
            return vec![];
        }
    };

    tokio_stream::wrappers::ReadDirStream::new(read_dir).filter_map(|item| match item {
        Ok(entry) => match entry.path().is_dir() {
            true => Some(entry.file_name().into_string().expect("String should be available as core string")),
            false => None
        },
        Err(_) => None
    }).collect::<Vec<String>>().await
}

pub async fn get_mounts_for_user(username: &String) -> Option<Vec<String>> {
    let mounts = get_mounts().await;
    if !user_exists(username).await {
        return None;
    }

    let mut for_user = vec![];
    for mount in mounts {
        match path_exists!(vec![validate_mount_dir().await, mount.to_string(), username.to_string()].join("/")) {
            true => for_user.push(mount),
            false => continue
        }
    }

    sort_strings_insensitive!(for_user);
    Some(for_user)
}

pub async fn get_users_for_mount(mount: &String) -> Option<Vec<String>> {
    let path = vec![validate_mount_dir().await, mount.to_string()].join("/");
    let read_dir = match tokio::fs::read_dir(path).await {
        Ok(dir) => dir,
        Err(err) => {
            log::error!("Failed to load mount dirs {}", err);
            return None;
        }
    };

    let tokio_data = tokio_stream::wrappers::ReadDirStream::new(read_dir).filter_map(|entry| match entry {
        Ok(entry) => Some(entry.file_name().into_string().expect("String should be available as core string")),
        Err(err) => {
            log::warn!("Failed to load {err}");
            None
        }
    }).collect::<Vec<String>>().await;

    let mut users_for_mount = vec![];

    for username in tokio_data {
        if user_exists(&username).await {
            users_for_mount.push(username)
        }
    }

    sort_strings_insensitive!(users_for_mount);
    Some(users_for_mount)
}

pub async fn mount_exists(mount: &String) -> bool {
    path_exists!(vec![validate_mount_dir().await, mount.to_string()].join("/"))
}

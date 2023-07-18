use tokio_stream::StreamExt;
use sheef_utils::sort_strings_insensitive;
use crate::{EmptyResult, validate_database_dir};
use crate::user::user_exists;

async fn validate_kill_dir() -> String {
    let path = vec![validate_database_dir().await, "kill".to_string()].join("/");
    let result = tokio::fs::create_dir_all(path.as_str()).await;
    if result.is_err() {
        panic!("Failed to create kill database dir {}", result.err().unwrap());
    }

    path
}

pub async fn create_kill(kill: &String) -> EmptyResult {
    let path = vec![validate_kill_dir().await, kill.to_string()].join("/");
    match tokio::fs::create_dir_all(path.as_str()).await {
        Ok(_) => Ok(()),
        Err(err) => {
            log::warn!("Failed to create kill dir ({}): {}", path, err);
            Err(())
        }
    }
}

pub async fn delete_kill(kill: &String) -> EmptyResult {
    let path = vec![validate_kill_dir().await, kill.to_string()].join("/");
    match tokio::fs::remove_dir_all(path.as_str()).await {
        Ok(_) => Ok(()),
        Err(err) => {
            log::warn!("Failed to delete kill dir ({}): {}", path, err);
            Err(())
        }
    }
}

pub async fn update_kill(kill: &String, new_name: &String) -> EmptyResult {
    let old_path = vec![validate_kill_dir().await, kill.to_string()].join("/");
    let new_path = vec![validate_kill_dir().await, new_name.to_string()].join("/");
    match tokio::fs::rename(old_path.as_str(), new_path.as_str()).await {
        Ok(_) => Ok(()),
        Err(err) => {
            log::warn!("Failed to move kill dir ({} -> {}): {}", old_path, new_path, err);
            Err(())
        }
    }
}

pub async fn activate_kill_for_user(kill: &String, username: &String) -> EmptyResult {
    let path = vec![validate_kill_dir().await, kill.to_string(), username.to_string()].join("/");
    match tokio::fs::File::create(path.as_str()).await {
        Ok(_) => Ok(()),
        Err(err) => {
            log::warn!("Failed to activate kill {} for user {} ({}): {}", kill, username, path, err);
            Err(())
        }
    }
}

pub async fn deactivate_kill_for_user(kill: &String, username: &String) -> EmptyResult {
    let path = vec![validate_kill_dir().await, kill.to_string(), username.to_string()].join("/");
    match tokio::fs::remove_file(path.as_str()).await {
        Ok(_) => Ok(()),
        Err(err) => {
            log::warn!("Failed to deactivate kill {} for user {} ({}): {}", kill, username, path, err);
            Err(())
        }
    }
}

pub async fn get_kills() -> Vec<String> {
    let read_dir = match tokio::fs::read_dir(validate_kill_dir().await).await {
        Ok(dir) => dir,
        Err(err) => {
            log::error!("Failed to load kill dirs {}", err);
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

pub async fn get_kills_for_user(username: &String) -> Option<Vec<String>> {
    let kills = get_kills().await;
    if !user_exists(username).await {
        return None;
    }

    let mut for_user = vec![];
    for kill in kills {
        match path_exists!(vec![validate_kill_dir().await, kill.to_string(), username.to_string()].join("/")) {
            true => for_user.push(kill),
            false => continue
        }
    }

    sort_strings_insensitive!(for_user);
    Some(for_user)
}

pub async fn get_users_for_kill(kill: &String) -> Option<Vec<String>> {
    let path = vec![validate_kill_dir().await, kill.to_string()].join("/");
    let read_dir = match tokio::fs::read_dir(path).await {
        Ok(dir) => dir,
        Err(err) => {
            log::error!("Failed to load kill dirs {}", err);
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

    let mut users_for_kill = vec![];

    for username in tokio_data {
        if user_exists(&username).await {
            users_for_kill.push(username)
        }
    }

    sort_strings_insensitive!(users_for_kill);
    Some(users_for_kill)
}

pub async fn kill_exists(kill: &String) -> bool {
    path_exists!(vec![validate_kill_dir().await, kill.to_string()].join("/"))
}

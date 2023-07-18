use tokio_stream::StreamExt;
use sheef_entities::{sheef_io_error, sheef_not_found_error};
use sheef_utils::sort_strings_insensitive;
use crate::{SheefErrorResult, SheefResult, validate_database_dir};
use crate::user::user_exists;

async fn validate_kill_dir() -> String {
    let path = vec![validate_database_dir().await, "kill".to_string()].join("/");
    let result = tokio::fs::create_dir_all(path.as_str()).await;
    if result.is_err() {
        panic!("Failed to create kill database dir {}", result.err().unwrap());
    }

    path
}

pub async fn create_kill(kill: &String) -> SheefErrorResult {
    let path = vec![validate_kill_dir().await, kill.to_string()].join("/");
    match tokio::fs::create_dir_all(path.as_str()).await {
        Ok(_) => Ok(()),
        Err(err) => {
            log::warn!("Failed to create kill dir ({}): {}", path, err);
            Err(sheef_io_error!("kill", "Failed to create kill dir"))
        }
    }
}

pub async fn delete_kill(kill: &String) -> SheefErrorResult {
    let path = vec![validate_kill_dir().await, kill.to_string()].join("/");
    match tokio::fs::remove_dir_all(path.as_str()).await {
        Ok(_) => Ok(()),
        Err(err) => {
            log::warn!("Failed to delete kill dir ({}): {}", path, err);
            Err(sheef_io_error!("kill", "Failed to delete kill dir"))
        }
    }
}

pub async fn update_kill(kill: &String, new_name: &String) -> SheefErrorResult {
    let old_path = vec![validate_kill_dir().await, kill.to_string()].join("/");
    let new_path = vec![validate_kill_dir().await, new_name.to_string()].join("/");
    match tokio::fs::rename(old_path.as_str(), new_path.as_str()).await {
        Ok(_) => Ok(()),
        Err(err) => {
            log::warn!("Failed to move kill dir ({} -> {}): {}", old_path, new_path, err);
            Err(sheef_io_error!("kill", "Failed to move kill dir"))
        }
    }
}

pub async fn activate_kill_for_user(kill: &String, username: &String) -> SheefErrorResult {
    let path = vec![validate_kill_dir().await, kill.to_string(), username.to_string()].join("/");
    match tokio::fs::File::create(path.as_str()).await {
        Ok(_) => Ok(()),
        Err(err) => {
            log::warn!("Failed to activate kill {} for user {} ({}): {}", kill, username, path, err);
            Err(sheef_io_error!("kill", "Failed to activate kill for user"))
        }
    }
}

pub async fn deactivate_kill_for_user(kill: &String, username: &String) -> SheefErrorResult {
    let path = vec![validate_kill_dir().await, kill.to_string(), username.to_string()].join("/");
    match tokio::fs::remove_file(path.as_str()).await {
        Ok(_) => Ok(()),
        Err(err) => {
            log::warn!("Failed to deactivate kill {} for user {} ({}): {}", kill, username, path, err);
            Err(sheef_io_error!("kill", "Failed to deactivate kill for user"))
        }
    }
}

pub async fn get_kills() -> SheefResult<Vec<String>> {
    let read_dir = match tokio::fs::read_dir(validate_kill_dir().await).await {
        Ok(dir) => dir,
        Err(err) => {
            log::error!("Failed to load kill dirs {}", err);
            return Err(sheef_io_error!("kill", "Failed to load kill dirs"));
        }
    };

    Ok(tokio_stream::wrappers::ReadDirStream::new(read_dir).filter_map(|item| match item {
        Ok(entry) => match entry.path().is_dir() {
            true => Some(entry.file_name().into_string().expect("String should be available as core string")),
            false => None
        },
        Err(_) => None
    }).collect::<Vec<String>>().await)
}

pub async fn get_kills_for_user(username: &String) -> SheefResult<Vec<String>> {
    let kills = match get_kills().await {
        Ok(kills) => kills,
        Err(err) => return Err(err)
    };
    if !user_exists(username).await {
        return Err(sheef_not_found_error!("kill", "User does not exist"));
    }

    let mut for_user = vec![];
    for kill in kills {
        match path_exists!(vec![validate_kill_dir().await, kill.to_string(), username.to_string()].join("/")) {
            true => for_user.push(kill),
            false => continue
        }
    }

    sort_strings_insensitive!(for_user);
    Ok(for_user)
}

pub async fn get_users_for_kill(kill: &String) -> SheefResult<Vec<String>> {
    let path = vec![validate_kill_dir().await, kill.to_string()].join("/");
    let read_dir = match tokio::fs::read_dir(path).await {
        Ok(dir) => dir,
        Err(err) => {
            log::error!("Failed to load kill dirs {}", err);
            return Err(sheef_io_error!("kill", "Failed to load kill dirs"));
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
    Ok(users_for_kill)
}

pub async fn kill_exists(kill: &String) -> bool {
    path_exists!(vec![validate_kill_dir().await, kill.to_string()].join("/"))
}

use sheef_api_entities::{Crafter, sheef_io_error, sheef_not_found_error};

use crate::{persist_entity, read_entity, read_entity_dir, SheefErrorResult, SheefResult, validate_database_dir};

async fn validate_crafter_dir() -> String {
    let path = vec![validate_database_dir().await, "crafter".to_string()].join("/");
    let result = tokio::fs::create_dir_all(path.as_str()).await;
    if result.is_err() {
        panic!("Failed to create crafter database dir {}", result.err().unwrap());
    }

    path
}

async fn get_user_crafter_dir(username: &String) -> SheefResult<String> {
    let path = vec![validate_crafter_dir().await, username.to_string()].join("/");
    match tokio::fs::create_dir_all(path.as_str()).await {
        Ok(_) => Ok(path),
        Err(err) => {
            log::warn!("Failed to create crafter dir for user {}: {}", username, err);
            Err(sheef_io_error!("crafter".to_string(), "Failed to create crafter dir for user".to_string()))
        }
    }
}

pub async fn create_crafter(username: &String, job: &String, level: &String) -> SheefResult<Crafter> {
    let crafter = Crafter {
        job: job.to_string(),
        level: level.to_string(),
    };

    let crafter_dir = match get_user_crafter_dir(username).await {
        Ok(dir) => dir,
        Err(err) => {
            log::warn!("Failed to get user crafter dir ({})", username);
            return Err(err);
        }
    };

    map_err!(persist_entity(crafter_dir, job, crafter).await, "crafter")
}

pub async fn get_crafter(username: &String, job: &String) -> SheefResult<Crafter> {
    let crafter_dir = match get_user_crafter_dir(username).await {
        Ok(dir) => dir,
        Err(err) => {
            log::warn!("Failed to get user crafter dir");
            return Err(err);
        }
    };

    map_err!(read_entity(crafter_dir, job).await, "crafter")
}

pub async fn get_crafters(username: &String) -> SheefResult<Vec<Crafter>> {
    let crafter_dir = match get_user_crafter_dir(username).await {
        Ok(dir) => dir,
        Err(err) => {
            log::warn!("Failed to get user crafter dir ({})", username);
            return Err(err);
        }
    };

    map_err!(read_entity_dir(crafter_dir).await, "crafter")
}

pub async fn update_crafter(username: &String, job: &String, level: &String, new_job: &String) -> SheefErrorResult {
    let mut crafter = match get_crafter(username, job).await {
        Ok(crafter) => crafter,
        Err(err) => {
            log::warn!("Crafter not found: {err}");
            return Err(sheef_not_found_error!("crafter", "Crafter not found"));
        }
    };
    let crafter_dir = match get_user_crafter_dir(username).await {
        Ok(dir) => dir,
        Err(err) => {
            log::warn!("Failed to get user crafter dir");
            return Err(err);
        }
    };

    crafter.level = level.to_string();
    crafter.job = new_job.to_string();
    match tokio::fs::rename(vec![crafter_dir.clone(), format!("{}.yaml", job)].join("/"), vec![crafter_dir.clone(), format!("{}.yaml", new_job)].join("/")).await {
        Ok(_) => {}
        Err(err) => {
            log::warn!("Failed to rename crafter: {err}");
            return Err(sheef_io_error!("crafter", "Failed to rename crafter"));
        }
    }

    map_err!(persist_entity(crafter_dir, new_job, crafter).await, "crafter").map(|_| ())
}

pub async fn delete_crafter(username: &String, job: &String) -> SheefErrorResult {
    let crafter_dir = match get_user_crafter_dir(username).await {
        Ok(dir) => dir,
        Err(err) => {
            log::warn!("Failed to get user crafter dir");
            return Err(err);
        }
    };
    match tokio::fs::remove_file(vec![crafter_dir, format!("{}.yaml", job)].join("/")).await {
        Ok(_) => Ok(()),
        Err(err) => {
            log::warn!("Failed to delete crafter {}", err);
            Err(sheef_io_error!("crafter", "Failed to delete crafter"))
        }
    }
}

pub async fn crafter_exists(username: &String, job: &String) -> bool {
    let crafter_dir = match get_user_crafter_dir(username).await {
        Ok(dir) => dir,
        Err(err) => {
            log::warn!("Failed to get user fighter dir: {}", err);
            return false;
        }
    };
    path_exists!(vec![crafter_dir, format!("{}.yaml", job)].join("/"))
}

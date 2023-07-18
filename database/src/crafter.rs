use sheef_entities::Crafter;
use crate::{EmptyResult, persist_entity, read_entity, read_entity_dir, validate_database_dir};

async fn validate_crafter_dir() -> String {
    let path = vec![validate_database_dir().await, "crafter".to_string()].join("/");
    let result = tokio::fs::create_dir_all(path.as_str()).await;
    if result.is_err() {
        panic!("Failed to create crafter database dir {}", result.err().unwrap());
    }

    path
}

async fn get_user_crafter_dir(username: &String) -> Option<String> {
    let path = vec![validate_crafter_dir().await, username.to_string()].join("/");
    match tokio::fs::create_dir_all(path.as_str()).await {
        Ok(_) => Some(path),
        Err(err) => {
            log::warn!("Failed to create crafter dir for user {}: {}", username, err);
            None
        }
    }
}

pub async fn create_crafter(username: &String, job: &String, level: &String) -> Option<Crafter> {
    let crafter = Crafter {
        job: job.to_string(),
        level: level.to_string(),
    };

    let crafter_dir = match get_user_crafter_dir(username).await {
        Some(dir) => dir,
        None => {
            log::warn!("Failed to get user crafter dir ({})", username);
            return None;
        }
    };

    match persist_entity(crafter_dir, job, crafter).await {
        Ok(crafter) => Some(crafter),
        Err(_) => None
    }
}

pub async fn get_crafter(username: &String, job: &String) -> Option<Crafter> {
    let crafter_dir = match get_user_crafter_dir(username).await {
        Some(dir) => dir,
        None => {
            log::warn!("Failed to get user crafter dir");
            return None;
        }
    };

    read_entity(crafter_dir, job).await
}

pub async fn get_crafters(username: &String) -> Option<Vec<Crafter>> {
    let crafter_dir = match get_user_crafter_dir(username).await {
        Some(dir) => dir,
        None => {
            log::warn!("Failed to get user crafter dir ({})", username);
            return None;
        }
    };

    read_entity_dir(crafter_dir).await
}

pub async fn update_crafter(username: &String, job: &String, level: &String, new_job: &String) -> EmptyResult {
    let mut crafter = match get_crafter(username, job).await {
        Some(crafter) => crafter,
        None => {
            log::warn!("Crafter not found");
            return Err(());
        }
    };
    let crafter_dir = match get_user_crafter_dir(username).await {
        Some(dir) => dir,
        None => {
            log::warn!("Failed to get user crafter dir");
            return Err(());
        }
    };

    crafter.level = level.to_string();
    crafter.job = new_job.to_string();
    match tokio::fs::rename(vec![crafter_dir.clone(), format!("{}.yaml", job)].join("/"), vec![crafter_dir.clone(), format!("{}.yaml", new_job)].join("/")).await {
        Ok(_) => {}
        Err(err) => {
            log::warn!("Failed to rename crafter: {err}");
            return Err(());
        }
    }

    match persist_entity(crafter_dir, new_job, crafter).await {
        Ok(_) => Ok(()),
        Err(_) => Err(())
    }
}

pub async fn delete_crafter(username: &String, job: &String) -> EmptyResult {
    let crafter_dir = match get_user_crafter_dir(username).await {
        Some(dir) => dir,
        None => {
            log::warn!("Failed to get user crafter dir");
            return Err(());
        }
    };
    match tokio::fs::remove_file(vec![crafter_dir, format!("{}.yaml", job)].join("/")).await {
        Ok(_) => Ok(()),
        Err(err) => {
            log::warn!("Failed to delete crafter {}", err);
            Err(())
        }
    }
}

pub async fn crafter_exists(username: &String, job: &String) -> bool {
    let crafter_dir = match get_user_crafter_dir(username).await {
        Some(dir) => dir,
        None => {
            log::warn!("Failed to get user fighter dir");
            return false;
        }
    };
    path_exists!(vec![crafter_dir, format!("{}.yaml", job)].join("/"))
}

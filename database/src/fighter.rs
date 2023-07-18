use sheef_entities::Fighter;
use crate::{EmptyResult, persist_entity, read_entity, read_entity_dir, validate_database_dir};

async fn validate_fighter_dir() -> String {
    let path = vec![validate_database_dir().await, "fighter".to_string()].join("/");
    let result = tokio::fs::create_dir_all(path.as_str()).await;
    if result.is_err() {
        panic!("Failed to create fighter database dir {}", result.err().unwrap());
    }

    path
}

async fn get_user_fighter_dir(username: &String) -> Option<String> {
    let path = vec![validate_fighter_dir().await, username.to_string()].join("/");
    match tokio::fs::create_dir_all(path.as_str()).await {
        Ok(_) => Some(path),
        Err(err) => {
            log::warn!("Failed to create fighter dir for user {}: {}", username, err);
            None
        }
    }
}

pub async fn create_fighter(username: &String, job: &String, level: &String, gear_score: &String) -> Option<Fighter> {
    let fighter = Fighter {
        job: job.to_string(),
        level: level.to_string(),
        gear_score: gear_score.to_string(),
    };

    let fighter_dir = match get_user_fighter_dir(username).await {
        Some(dir) => dir,
        None => {
            log::warn!("Failed to get user fighter dir ({})", username);
            return None;
        }
    };

    match persist_entity(fighter_dir, job, fighter).await {
        Ok(fighter) => Some(fighter),
        Err(_) => None
    }
}

pub async fn get_fighter(username: &String, job: &String) -> Option<Fighter> {
    let fighter_dir = match get_user_fighter_dir(username).await {
        Some(dir) => dir,
        None => {
            log::warn!("Failed to get user fighter dir");
            return None;
        }
    };

    read_entity(fighter_dir, job).await
}

pub async fn get_fighters(username: &String) -> Option<Vec<Fighter>> {
    let fighter_dir = match get_user_fighter_dir(username).await {
        Some(dir) => dir,
        None => {
            log::warn!("Failed to get user fighter dir ({})", username);
            return None;
        }
    };

    read_entity_dir(fighter_dir).await
}

pub async fn update_fighter(username: &String, job: &String, level: &String, gear_score: &String, new_job: &String) -> EmptyResult {
    let mut fighter = match get_fighter(username, job).await {
        Some(fighter) => fighter,
        None => {
            log::warn!("Fighter not found");
            return Err(());
        }
    };
    let fighter_dir = match get_user_fighter_dir(username).await {
        Some(dir) => dir,
        None => {
            log::warn!("Failed to get user fighter dir");
            return Err(());
        }
    };

    fighter.level = level.to_string();
    fighter.job = new_job.to_string();
    fighter.gear_score = gear_score.to_string();

    match tokio::fs::rename(vec![fighter_dir.clone(), format!("{}.yaml", job)].join("/"), vec![fighter_dir.clone(), format!("{}.yaml", new_job)].join("/")).await {
        Ok(_) => {}
        Err(err) => {
            log::warn!("Failed to rename fighter: {err}");
            return Err(());
        }
    }

    match persist_entity(fighter_dir, new_job, fighter).await {
        Ok(_) => Ok(()),
        Err(_) => Err(())
    }
}

pub async fn delete_fighter(username: &String, job: &String) -> EmptyResult {
    let fighter_dir = match get_user_fighter_dir(username).await {
        Some(dir) => dir,
        None => {
            log::warn!("Failed to get user fighter dir");
            return Err(());
        }
    };
    match tokio::fs::remove_file(vec![fighter_dir, format!("{}.yaml", job)].join("/")).await {
        Ok(_) => Ok(()),
        Err(err) => {
            log::warn!("Failed to delete fighter {}", err);
            Err(())
        }
    }
}

pub async fn fighter_exists(username: &String, job: &String) -> bool {
    let fighter_dir = match get_user_fighter_dir(username).await {
        Some(dir) => dir,
        None => {
            log::warn!("Failed to get user fighter dir");
            return false;
        }
    };

    path_exists!(vec![fighter_dir, format!("{}.yaml", job)].join("/"))
}
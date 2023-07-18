use sheef_entities::{Fighter, sheef_io_error, sheef_not_found_error};
use crate::{SheefErrorResult, persist_entity, read_entity, read_entity_dir, validate_database_dir, SheefResult};

async fn validate_fighter_dir() -> String {
    let path = vec![validate_database_dir().await, "fighter".to_string()].join("/");
    let result = tokio::fs::create_dir_all(path.as_str()).await;
    if result.is_err() {
        panic!("Failed to create fighter database dir {}", result.err().unwrap());
    }

    path
}

async fn get_user_fighter_dir(username: &String) -> SheefResult<String> {
    let path = vec![validate_fighter_dir().await, username.to_string()].join("/");
    match tokio::fs::create_dir_all(path.as_str()).await {
        Ok(_) => Ok(path),
        Err(err) => {
            log::warn!("Failed to create fighter dir for user {}: {}", username, err);
            Err(sheef_io_error!("fighter", "Failed to create fighter dir for user"))
        }
    }
}

pub async fn create_fighter(username: &String, job: &String, level: &String, gear_score: &String) -> SheefResult<Fighter> {
    let fighter = Fighter {
        job: job.to_string(),
        level: level.to_string(),
        gear_score: gear_score.to_string(),
    };

    let fighter_dir = match get_user_fighter_dir(username).await {
        Ok(dir) => dir,
        Err(err) => {
            log::warn!("Failed to get user fighter dir ({})", username);
            return Err(err);
        }
    };

    map_err!(persist_entity(fighter_dir, job, fighter).await, "fighter").map(|fighter| fighter)
}

pub async fn get_fighter(username: &String, job: &String) -> SheefResult<Fighter> {
    let fighter_dir = match get_user_fighter_dir(username).await {
        Ok(dir) => dir,
        Err(err) => {
            log::warn!("Failed to get user fighter dir");
            return Err(err);
        }
    };

    map_err!(read_entity(fighter_dir, job).await, "fighter")
}

pub async fn get_fighters(username: &String) -> SheefResult<Vec<Fighter>> {
    let fighter_dir = match get_user_fighter_dir(username).await {
        Ok(dir) => dir,
        Err(err) => {
            log::warn!("Failed to get user fighter dir ({})", username);
            return Err(err);
        }
    };

    map_err!(read_entity_dir(fighter_dir).await, "fighter")
}

pub async fn update_fighter(username: &String, job: &String, level: &String, gear_score: &String, new_job: &String) -> SheefErrorResult {
    let mut fighter = match get_fighter(username, job).await {
        Ok(fighter) => fighter,
        Err(err) => {
            log::warn!("Fighter not found: {err}");
            return Err(sheef_not_found_error!("fighter", "Fighter not found"));
        }
    };
    let fighter_dir = match get_user_fighter_dir(username).await {
        Ok(dir) => dir,
        Err(err) => {
            log::warn!("Failed to get user fighter dir");
            return Err(err);
        }
    };

    fighter.level = level.to_string();
    fighter.job = new_job.to_string();
    fighter.gear_score = gear_score.to_string();

    match tokio::fs::rename(vec![fighter_dir.clone(), format!("{}.yaml", job)].join("/"), vec![fighter_dir.clone(), format!("{}.yaml", new_job)].join("/")).await {
        Ok(_) => {}
        Err(err) => {
            log::warn!("Failed to rename fighter: {err}");
            return Err(sheef_io_error!("fighter", "Failed to rename fighter"));
        }
    }

    map_err!(persist_entity(fighter_dir, new_job, fighter).await, "fighter").map(|_| ())
}

pub async fn delete_fighter(username: &String, job: &String) -> SheefErrorResult {
    let fighter_dir = match get_user_fighter_dir(username).await {
        Ok(dir) => dir,
        Err(err) => {
            log::warn!("Failed to get user fighter dir");
            return Err(err);
        }
    };

    match tokio::fs::remove_file(vec![fighter_dir, format!("{}.yaml", job)].join("/")).await {
        Ok(_) => Ok(()),
        Err(err) => {
            log::warn!("Failed to delete fighter {}", err);
            Err(sheef_io_error!("fighter", "Failed to delete fighter"))
        }
    }
}

pub async fn fighter_exists(username: &String, job: &String) -> bool {
    let fighter_dir = match get_user_fighter_dir(username).await {
        Ok(dir) => dir,
        Err(err) => {
            log::warn!("Failed to get user fighter dir: {err}");
            return false;
        }
    };

    path_exists!(vec![fighter_dir, format!("{}.yaml", job)].join("/"))
}
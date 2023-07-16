use std::fs::{remove_file, rename};
use log::warn;
use sheef_entities::Crafter;
use crate::{EmptyResult, persist_entity, read_entity, read_entity_dir, validate_database_dir};

fn validate_crafter_dir() -> String {
    let path = vec![validate_database_dir(), "crafter".to_string()].join("/");
    let result = std::fs::create_dir_all(path.as_str());
    if result.is_err() {
        panic!("Failed to create crafter database dir {}", result.err().unwrap());
    }

    path
}

fn get_user_crafter_dir(username: &String) -> Option<String> {
    let path = vec![validate_crafter_dir(), username.to_string()].join("/");
    match std::fs::create_dir_all(path.as_str()) {
        Ok(_) => Some(path),
        Err(err) => {
            warn!("Failed to create crafter dir for user {}: {}", username, err);
            None
        }
    }
}

pub fn create_crafter(username: &String, job: &String, level: &String) -> Option<Crafter> {
    let crafter = Crafter {
        job: job.to_string(),
        level: level.to_string(),
    };

    let crafter_dir = match get_user_crafter_dir(username) {
        Some(dir) => dir,
        None => {
            warn!("Failed to get user crafter dir ({})", username);
            return None;
        }
    };

    match persist_entity(crafter_dir, job, crafter) {
        Ok(crafter) => Some(crafter),
        Err(_) => None
    }
}

pub fn get_crafter(username: &String, job: &String) -> Option<Crafter> {
    let crafter_dir = match get_user_crafter_dir(username) {
        Some(dir) => dir,
        None => {
            warn!("Failed to get user crafter dir");
            return None;
        }
    };

    read_entity(crafter_dir, job)
}

pub fn get_crafters(username: &String) -> Option<Vec<Crafter>> {
    let crafter_dir = match get_user_crafter_dir(username) {
        Some(dir) => dir,
        None => {
            warn!("Failed to get user crafter dir ({})", username);
            return None;
        }
    };

    read_entity_dir(crafter_dir)
}

pub fn update_crafter(username: &String, job: &String, level: &String, new_job: &String) -> EmptyResult {
    let mut crafter = match get_crafter(username, job) {
        Some(crafter) => crafter,
        None => {
            warn!("Crafter not found");
            return Err(());
        }
    };
    let crafter_dir = match get_user_crafter_dir(username) {
        Some(dir) => dir,
        None => {
            warn!("Failed to get user crafter dir");
            return Err(());
        }
    };

    crafter.level = level.to_string();
    crafter.job = new_job.to_string();
    let _ = rename(vec![crafter_dir.clone(), format!("{}.yaml", job)].join("/"), vec![crafter_dir.clone(), format!("{}.yaml", new_job)].join("/"));

    match persist_entity(crafter_dir, new_job, crafter) {
        Ok(_) => Ok(()),
        Err(_) => Err(())
    }
}

pub fn delete_crafter(username: &String, job: &String) -> EmptyResult {
    let crafter_dir = match get_user_crafter_dir(username) {
        Some(dir) => dir,
        None => {
            warn!("Failed to get user crafter dir");
            return Err(());
        }
    };
    match remove_file(vec![crafter_dir, format!("{}.yaml", job)].join("/")) {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("Failed to delete crafter {}", err);
            Err(())
        }
    }
}

pub fn crafter_exists(username: &String, job: &String) -> bool {
    let crafter_dir = match get_user_crafter_dir(username) {
        Some(dir) => dir,
        None => {
            warn!("Failed to get user fighter dir");
            return false;
        }
    };
    path_exists!(vec![crafter_dir, format!("{}.yaml", job)].join("/"))
}

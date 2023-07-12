use std::env::join_paths;
use std::fs::remove_file;
use log::warn;
use serde::{Deserialize, Serialize};
use crate::database::{EmptyResult, persist_entity, read_entity, read_entity_dir, validate_database_dir};

#[derive(Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct Crafter {
    pub job: String,
    pub level: String,
}

fn validate_crafter_dir() -> String {
    let path = join_paths(vec![validate_database_dir(), "crafter".to_string()]).expect("Paths should be able to join").into_string().expect("String should be available as core string");
    let result = std::fs::create_dir_all(path.as_str());
    if result.is_err() {
        panic!("Failed to create crafter database dir {}", result.err().unwrap());
    }

    path
}

fn get_user_crafter_dir(username: &String) -> Option<String> {
    let path = join_paths(vec![validate_crafter_dir(), username.to_string()]).expect("Paths should be able to join").into_string().expect("String should be available as core string");
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

pub fn get_crafters(username: &String) -> Option<impl Iterator<Item=Crafter>> {
    let crafter_dir = match get_user_crafter_dir(username) {
        Some(dir) => dir,
        None => {
            warn!("Failed to get user crafter dir ({})", username);
            return None;
        }
    };

    read_entity_dir(crafter_dir)
}

pub fn update_crafter(username: &String, job: &String, level: &String) -> EmptyResult {
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

    match persist_entity(crafter_dir, job, crafter) {
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
    match remove_file(join_paths(vec![crafter_dir, format!("{}.yaml", job)]).expect("Paths should be able to join")) {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("Failed to delete crafter {}", err);
            Err(())
        }
    }
}

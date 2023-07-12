use std::env::join_paths;
use std::fs::remove_file;
use log::warn;
use serde::{Deserialize, Serialize};
use crate::database::{EmptyResult, persist_entity, read_entity, read_entity_dir, validate_database_dir};

#[derive(Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Fighter {
    pub job: String,
    pub level: String,
    pub gear_score: String
}

fn validate_fighter_dir() -> String {
    let path = join_paths(vec![validate_database_dir(), "fighter".to_string()]).expect("Paths should be able to join").into_string().expect("String should be available as core string");
    let result = std::fs::create_dir_all(path.as_str());
    if result.is_err() {
        panic!("Failed to create fighter database dir {}", result.err().unwrap());
    }

    path
}

fn get_user_fighter_dir(username: &String) -> Option<String> {
    let path = join_paths(vec![validate_fighter_dir(), username.to_string()]).expect("Paths should be able to join").into_string().expect("String should be available as core string");
    match std::fs::create_dir_all(path.as_str()) {
        Ok(_) => Some(path),
        Err(err) => {
            warn!("Failed to create fighter dir for user {}: {}", username, err);
            None
        }
    }
}

pub fn create_fighter(username: &String, job: &String, level: &String, gear_score: &String) -> Option<Fighter> {
    let fighter = Fighter {
        job: job.to_string(),
        level: level.to_string(),
        gear_score: gear_score.to_string()
    };

    let fighter_dir = match get_user_fighter_dir(username) {
        Some(dir) => dir,
        None => {
            warn!("Failed to get user fighter dir ({})", username);
            return None;
        }
    };

    match persist_entity(fighter_dir, job, fighter) {
        Ok(fighter) => Some(fighter),
        Err(_) => None
    }
}

pub fn get_fighter(username: &String, job: &String) -> Option<Fighter> {
    let fighter_dir = match get_user_fighter_dir(username) {
        Some(dir) => dir,
        None => {
            warn!("Failed to get user fighter dir");
            return None;
        }
    };

    read_entity(fighter_dir, job)
}

pub fn get_fighters(username: &String) -> Option<impl Iterator<Item=Fighter>> {
    let fighter_dir = match get_user_fighter_dir(username) {
        Some(dir) => dir,
        None => {
            warn!("Failed to get user fighter dir ({})", username);
            return None;
        }
    };

    read_entity_dir(fighter_dir)
}

pub fn update_fighter(username: &String, job: &String, level: &String, gear_score: &String) -> EmptyResult {
    let mut fighter = match get_fighter(username, job) {
        Some(fighter) => fighter,
        None => {
            warn!("Fighter not found");
            return Err(());
        }
    };
    let fighter_dir = match get_user_fighter_dir(username) {
        Some(dir) => dir,
        None => {
            warn!("Failed to get user fighter dir");
            return Err(());
        }
    };
    fighter.level = level.to_string();
    fighter.gear_score = gear_score.to_string();

    match persist_entity(fighter_dir, job, fighter) {
        Ok(_) => Ok(()),
        Err(_) => Err(())
    }
}

pub fn delete_fighter(username: &String, job: &String) -> EmptyResult {
    let fighter_dir = match get_user_fighter_dir(username) {
        Some(dir) => dir,
        None => {
            warn!("Failed to get user fighter dir");
            return Err(());
        }
    };
    match remove_file(join_paths(vec![fighter_dir, format!("{}.yaml", job)]).expect("Paths should be able to join")) {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("Failed to delete fighter {}", err);
            Err(())
        }
    }
}

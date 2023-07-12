use std::env::join_paths;
use std::fs::{File, remove_file};
use std::time::{SystemTime, UNIX_EPOCH};
use bcrypt::verify;
use log::warn;
use serde::{Deserialize, Serialize};
use sha2::{Sha512, Digest};
use crate::database::user::{get_user, validate_user_dir};
use base64::{Engine, engine::general_purpose};

#[derive(Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct Token {}

fn validate_token_dir() -> String {
    let path = join_paths(vec![validate_user_dir(), "token".to_string()]).expect("Paths should be able to join").into_string().expect("String should be available as core string");
    let result = std::fs::create_dir_all(path.as_str());
    if result.is_err() {
        panic!("Failed to create token database dir {}", result.err().unwrap());
    }

    path
}

fn get_user_token_dir(username: String) -> Option<String> {
    let path = join_paths(vec![validate_token_dir(), username.to_string()]).expect("Paths should be able to join").into_string().expect("String should be available as core string");
    match std::fs::create_dir_all(path.as_str()) {
        Ok(_) => Some(path),
        Err(err) => {
            warn!("Failed to create token dir for user {}: {}", username, err);
            None
        }
    }
}

pub fn validate_auth_and_create_token(username: &String, password: &String) -> Option<String> {
    let user = match get_user(username) {
        Some(user) => user,
        None => return None
    };

    let is_valid = verify(password, user.password.as_str()).unwrap_or(false);

    if !is_valid {
        return None;
    }

    let mut sha = Sha512::new();
    sha.update(SystemTime::now().duration_since(UNIX_EPOCH).expect("Time not working").as_micros().to_string().as_bytes());
    let result = &sha.finalize()[..];
    let token = &general_purpose::STANDARD.encode(&result);

    let token_dir = match get_user_token_dir(username.to_string()) {
        Some(dir) => dir,
        None => {
            warn!("Failed to get user token dir");
            return None;
        }
    };

    match File::create(join_paths(vec![token_dir, token.to_string()]).expect("Paths should be able to join")) {
        Ok(_) => Some(token.to_string()),
        Err(err) => {
            warn!("Failed to create file containing the token for user {}: {}", username, err);
            None
        }
    }
}

pub fn remove_token(username: &String, token: &String) {
    let token_dir = match get_user_token_dir(username.into()) {
        Some(dir) => dir,
        None => {
            warn!("Failed to get user token dir");
            return;
        }
    };

    _ = remove_file(join_paths(vec![token_dir, token.to_string()]).expect("Paths should be able to join"));
}

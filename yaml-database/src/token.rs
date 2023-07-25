use std::time::{SystemTime, UNIX_EPOCH};

use base64::{Engine, engine::general_purpose};
use bcrypt::verify;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};

use sheef_yaml_entities::{sheef_io_error, sheef_not_found_error, sheef_validation_error};
use sheef_yaml_entities::authentication::LoginResult;
use sheef_yaml_entities::user::User;

use crate::SheefResult;
use crate::user::{get_user, validate_user_dir};

#[derive(Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct Token {}

async fn validate_token_dir() -> String {
    let path = vec![validate_user_dir().await, "token".to_string()].join("/");
    let result = tokio::fs::create_dir_all(path.as_str()).await;
    if result.is_err() {
        panic!("Failed to create token database dir {}", result.err().unwrap());
    }

    path
}

pub(crate) async fn get_user_token_dir(username: String) -> SheefResult<String> {
    let path = vec![validate_token_dir().await, username.to_string()].join("/");
    match tokio::fs::create_dir_all(path.as_str()).await {
        Ok(_) => Ok(path),
        Err(err) => {
            log::warn!("Failed to create token dir for user {}: {}", username, err);
            Err(sheef_io_error!("token", "Failed to create token dir for user"))
        }
    }
}

pub async fn validate_auth_and_create_token(username: &String, password: &String) -> SheefResult<LoginResult> {
    let user = match get_user(username).await {
        Ok(user) => user,
        Err(err) => {
            log::warn!("Failed to load user {username}: {err}");
            return Err(sheef_not_found_error!("token", "User not found"));
        }
    };

    let is_valid = verify(password, user.password.as_str()).unwrap_or(false);

    if !is_valid {
        return Err(sheef_validation_error!("token", "Password is invalid"));
    }

    let mut sha = Sha512::new();
    sha.update(SystemTime::now().duration_since(UNIX_EPOCH).expect("Time not working").as_micros().to_string().as_bytes());
    let result = &sha.finalize()[..];
    let token = &general_purpose::URL_SAFE.encode(result);

    let token_dir = match get_user_token_dir(username.to_string()).await {
        Ok(dir) => dir,
        Err(err) => {
            log::warn!("Failed to get user token dir");
            return Err(err);
        }
    };

    match tokio::fs::File::create(vec![token_dir, token.to_string()].join("/")).await {
        Ok(_) => Ok(LoginResult {
            token: format!("{}/{}", username, token),
            user: user.to_web_user(),
        }),
        Err(err) => {
            log::warn!("Failed to create file containing the token for user {}: {}", username, err);
            Err(sheef_io_error!("token", "Failed to create file containing the token for user"))
        }
    }
}

pub async fn remove_token(username: &String, token: &String) {
    let token_dir = match get_user_token_dir(username.into()).await {
        Ok(dir) => dir,
        Err(err) => {
            log::warn!("Failed to get user token dir: {err}");
            return;
        }
    };

    let token_path = vec![token_dir, token.to_string()].join("/");
    let res = tokio::fs::remove_file(&token_path).await;
    if let Err(err) = res {
        log::warn!("Failed to delete token ({}): {}", token_path, err);
    }
}

pub async fn get_user_by_token(username: &String, token: &String) -> SheefResult<User> {
    let token_dir = match get_user_token_dir(username.to_string()).await {
        Ok(path) => path,
        Err(err) => return Err(err),
    };

    if path_exists!(vec![token_dir, token.to_string()].join("/")) {
        get_user(username).await
    } else {
        Err(sheef_not_found_error!("token", "Token not found"))
    }
}

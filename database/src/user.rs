use sheef_entities::{sheef_io_error, sheef_not_found_error, sheef_serialization_error, sheef_unknown_error};
use sheef_entities::user::User;
use crate::{SheefErrorResult, persist_entity, read_entity, read_entity_dir, read_entity_sync, validate_database_dir, validate_database_dir_sync, SheefResult};
use crate::token::get_user_token_dir;

pub(crate) async fn validate_user_dir() -> String {
    let path = vec![validate_database_dir().await, "user".to_string()].join("/");
    let result = tokio::fs::create_dir_all(path.as_str()).await;
    if result.is_err() {
        panic!("Failed to create user database dir {}", result.err().unwrap());
    }

    path
}

pub(crate) fn validate_user_dir_sync() -> String {
    let path = vec![validate_database_dir_sync(), "user".to_string()].join("/");
    let result = std::fs::create_dir_all(path.as_str());
    if result.is_err() {
        panic!("Failed to create user database dir {}", result.err().unwrap());
    }

    path
}

pub async fn create_user(username: &String, password: &String, is_mod: bool, is_main_group: bool, gear_level: &String, job: &String, is_hidden: bool) -> SheefResult<User> {
    let mut user = User {
        username: username.to_string(),
        password: "".to_string(),
        is_main_group,
        is_mod,
        gear_level: gear_level.to_string(),
        job: job.to_string(),
        is_hidden,
    };
    if let Err(err) = user.set_password(password) {
        log::warn!("Failed to set user password {}", err);
        return Err(sheef_serialization_error!("user", "Failed to set password for user"));
    }

    match persist_entity(validate_user_dir().await, username, user).await {
        Ok(user) => Ok(user),
        Err(mut err) => {
            err.entity_type = "user".to_string();
            Err(err)
        }
    }
}

pub async fn delete_user(username: &String) -> SheefErrorResult {
    match tokio::fs::remove_file(vec![validate_user_dir().await, format!("{}.yaml", username)].join("/")).await {
        Ok(_) => Ok(()),
        Err(err) => {
            log::warn!("Failed to delete user {}", err);
            Err(sheef_io_error!("user", "Failed to delete user"))
        }
    }
}

pub async fn get_user(username: &String) -> SheefResult<User> {
    map_err!(read_entity(validate_user_dir().await, username).await, "user")
}

pub fn get_user_sync(username: &String) -> SheefResult<User> {
    map_err!(read_entity_sync(validate_user_dir_sync(), username), "user")
}

pub async fn get_users() -> SheefResult<Vec<User>> {
    match read_entity_dir::<User>(validate_user_dir().await).await {
        Ok(users) => Ok(users.into_iter().filter(|user| !user.is_hidden).collect::<Vec<User>>()),
        Err(err) => Err(sheef_unknown_error!("user", err.message))
    }
}

pub async fn change_mod_status(username: &String, is_mod: bool) -> SheefErrorResult {
    let mut user = match get_user(username).await {
        Ok(user) => user,
        Err(err) => {
            log::warn!("User {} not found", username);
            return Err(sheef_not_found_error!("user", err.message));
        }
    };

    user.is_mod = is_mod;
    map_err!(persist_entity(validate_user_dir().await, username, user).await, "user").map(|_| ())
}

pub async fn change_main_group(username: &String, is_main_group: bool) -> SheefErrorResult {
    let mut user = match get_user(username).await {
        Ok(user) => user,
        Err(err) => {
            log::warn!("User {} not found", username);
            return Err(sheef_not_found_error!("user", err.message));
        }
    };

    user.is_main_group = is_main_group;
    map_err!(persist_entity(validate_user_dir().await, username, user).await, "user").map(|_| ())
}

pub async fn change_password(username: &String, password: &String) -> SheefErrorResult {
    let mut user = match get_user(username).await {
        Ok(user) => user,
        Err(err) => {
            log::warn!("User {} not found", username);
            return Err(sheef_not_found_error!("user", err.message));
        }
    };

    if let Err(err) = user.set_password(password) {
        log::warn!("Failed to set password for user {}: {}", username, err);
        return Err(sheef_serialization_error!("user", "Failed to set password for user"));
    }

    match persist_entity(validate_user_dir().await, username, user).await {
        Ok(_) => {
            match tokio::fs::remove_dir_all(get_user_token_dir(username.to_string()).await.expect("User token dir cannot be empty")).await {
                Ok(_) => {}
                Err(err) => log::warn!("Failed to remove the token directory for {username}: {err}"),
            }
            Ok(())
        }
        Err(err) => Err(err)
    }
}

pub async fn update_me(username: &String, job: &String, gear_level: &String) -> SheefErrorResult {
    let mut user = match get_user(username).await {
        Ok(user) => user,
        Err(err) => {
            log::warn!("User {} not found", username);
            return Err(sheef_not_found_error!("user", err.message));
        }
    };

    user.gear_level = gear_level.to_string();
    user.job = job.to_string();

    map_err!(persist_entity(validate_user_dir().await, username, user).await, "user").map(|_| ())
}

pub async fn user_exists(username: &String) -> bool {
    path_exists!(vec![validate_user_dir().await, format!("{}.yaml", username)].join("/"))
}

pub enum PasswordError {
    WrongPassword,
    UserNotFound,
    UnknownError,
}

pub async fn change_my_password(username: &String, old_password: &String, new_password: &String) -> Result<(), PasswordError> {
    if let Ok(user) = get_user(username).await {
        if user.validate_password(old_password) {
            change_password(username, new_password).await.map_err(|_| PasswordError::UnknownError)
        } else {
            Err(PasswordError::WrongPassword)
        }
    } else {
        Err(PasswordError::UserNotFound)
    }
}

use sheef_entities::user::User;
use crate::{EmptyResult, persist_entity, read_entity, read_entity_dir, read_entity_sync, validate_database_dir, validate_database_dir_sync};
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

pub async fn create_user(username: &String, password: &String, is_mod: bool, is_main_group: bool, gear_level: &String, job: &String, is_hidden: bool) -> Option<User> {
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
        return None;
    }

    match persist_entity(validate_user_dir().await, username, user).await {
        Ok(user) => Some(user),
        _ => None
    }
}

pub async fn delete_user(username: &String) -> EmptyResult {
    match tokio::fs::remove_file(vec![validate_user_dir().await, format!("{}.yaml", username)].join("/")).await {
        Ok(_) => Ok(()),
        Err(err) => {
            log::warn!("Failed to delete user {}", err);
            Err(())
        }
    }
}

pub async fn get_user(username: &String) -> Option<User> {
    read_entity(validate_user_dir().await, username).await
}

pub fn get_user_sync(username: &String) -> Option<User> {
    read_entity_sync(validate_user_dir_sync(), username)
}

pub async fn get_users() -> Option<Vec<User>> {
    Some(read_entity_dir::<User>(validate_user_dir().await).await.unwrap().into_iter().filter(|user| !user.is_hidden).collect::<Vec<User>>())
}

pub async fn change_mod_status(username: &String, is_mod: bool) -> EmptyResult {
    let mut user = match get_user(username).await {
        Some(user) => user,
        None => {
            log::warn!("User {} not found", username);
            return Err(());
        }
    };
    user.is_mod = is_mod;
    match persist_entity(validate_user_dir().await, username, user).await {
        Ok(_) => Ok(()),
        Err(_) => Err(())
    }
}

pub async fn change_main_group(username: &String, is_main_group: bool) -> EmptyResult {
    let mut user = match get_user(username).await {
        Some(user) => user,
        None => {
            log::warn!("User {} not found", username);
            return Err(());
        }
    };
    user.is_main_group = is_main_group;
    match persist_entity(validate_user_dir().await, username, user).await {
        Ok(_) => Ok(()),
        Err(_) => Err(())
    }
}

pub async fn change_password(username: &String, password: &String) -> EmptyResult {
    let mut user = match get_user(username).await {
        Some(user) => user,
        None => {
            log::warn!("User {} not found", username);
            return Err(());
        }
    };

    if let Err(err) = user.set_password(password) {
        log::warn!("Failed to set password for user {}: {}", username, err);
        return Err(());
    }

    match persist_entity(validate_user_dir().await, username, user).await {
        Ok(_) => {
            match tokio::fs::remove_dir_all(get_user_token_dir(username.to_string()).await.expect("User token dir cannot be empty")).await {
                Ok(_) => {}
                Err(err) => log::warn!("Failed to remove the token directory for {username}: {err}"),
            }
            Ok(())
        }
        Err(_) => Err(())
    }
}

pub async fn update_me(username: &String, job: &String, gear_level: &String) -> EmptyResult {
    let mut user = match get_user(username).await {
        Some(user) => user,
        None => {
            log::warn!("User {} not found", username);
            return Err(());
        }
    };

    user.gear_level = gear_level.to_string();
    user.job = job.to_string();

    match persist_entity(validate_user_dir().await, username, user).await {
        Ok(_) => Ok(()),
        Err(_) => Err(())
    }
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
    if let Some(user) = get_user(username).await {
        if user.validate_password(old_password) {
            change_password(username, new_password).await.map_err(|_| PasswordError::UnknownError)
        } else {
            Err(PasswordError::WrongPassword)
        }
    } else {
        Err(PasswordError::UserNotFound)
    }
}

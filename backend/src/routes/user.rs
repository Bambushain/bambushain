use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;
use sheef_database::user::{PasswordError, user_exists};
use sheef_entities::{sheef_exists_already_error, sheef_insufficient_rights_error, sheef_not_found_error, sheef_unknown_error, sheef_validation_error, UpdateProfile, User, user};
use sheef_entities::authentication::{ChangeMyPassword, ChangePassword};

#[derive(Deserialize)]
pub struct UserPathInfo {
    pub username: String,
}

macro_rules! prevent_me {
    ($req:ident, $username:expr, $error_message:expr) => {
        {
            if username!($req) == $username {
                return conflict!(sheef_validation_error!("user", $error_message));
            };
        }
    };
}

pub async fn get_users() -> HttpResponse {
    ok_or_error!(sheef_database::user::get_users().await.map(|users| users.into_iter().map(|u| User {
        username: u.username,
        job: u.job,
        gear_level: u.gear_level,
        is_mod: u.is_mod,
        is_main_group: u.is_main_group,
    }).collect::<Vec<User>>()))
}

pub async fn get_user(info: web::Path<UserPathInfo>) -> HttpResponse {
    ok_or_error!(sheef_database::user::get_user(&info.username).await.map(|u| u.to_web_user()))
}

pub async fn create_user(user: web::Json<user::User>) -> HttpResponse {
    if user_exists(&user.username).await {
        return conflict!(sheef_exists_already_error!("user", "A user with the name already exists"));
    }

    created_or_error!(sheef_database::user::create_user(&user.username, &user.password, user.is_mod, user.is_main_group, &user.gear_level, &user.job, user.is_hidden).await.map(|u| u.to_web_user()))
}

pub async fn delete_user(info: web::Path<UserPathInfo>, req: HttpRequest) -> HttpResponse {
    prevent_me!(req, info.username, "You cannot delete yourself");
    if !user_exists(&info.username).await {
        return not_found!(sheef_not_found_error!("user", "The user was not found"));
    }

    no_content_or_error!(sheef_database::user::delete_user(&info.username).await)
}

pub async fn add_mod_user(info: web::Path<UserPathInfo>, req: HttpRequest) -> HttpResponse {
    prevent_me!(req, info.username, "You cannot make yourself mod");
    if !user_exists(&info.username).await {
        return not_found!(sheef_not_found_error!("user", "The user was not found"));
    }

    no_content_or_error!(sheef_database::user::change_mod_status(&info.username, true).await)
}

pub async fn remove_mod_user(info: web::Path<UserPathInfo>, req: HttpRequest) -> HttpResponse {
    prevent_me!(req, info.username, "You cannot revoke your own mod rights");
    if !user_exists(&info.username).await {
        return not_found!(sheef_not_found_error!("user", "The user was not found"));
    }

    no_content_or_error!(sheef_database::user::change_mod_status(&info.username, false).await)
}

pub async fn add_main_group_user(info: web::Path<UserPathInfo>) -> HttpResponse {
    if !user_exists(&info.username).await {
        return not_found!(sheef_not_found_error!("user", "The user was not found"));
    }

    no_content_or_error!(sheef_database::user::change_main_group(&info.username, true).await)
}

pub async fn remove_main_group_user(info: web::Path<UserPathInfo>) -> HttpResponse {
    if !user_exists(&info.username).await {
        return not_found!(sheef_not_found_error!("user", "The user was not found"));
    }

    no_content_or_error!(sheef_database::user::change_main_group(&info.username, false).await)
}

pub async fn change_password(info: web::Path<UserPathInfo>, body: web::Json<ChangePassword>, req: HttpRequest) -> HttpResponse {
    prevent_me!(req, info.username, "You cannot change your own password using this endpoint");
    if !user_exists(&info.username).await {
        return not_found!(sheef_not_found_error!("user", "The user was not found"));
    }

    no_content_or_error!(sheef_database::user::change_password(&info.username.to_string(), &body.new_password).await)
}

pub async fn change_my_password(body: web::Json<ChangeMyPassword>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    match sheef_database::user::change_my_password(&username, &body.old_password, &body.new_password).await {
        Ok(_) => no_content!(),
        Err(PasswordError::WrongPassword) => forbidden!(sheef_insufficient_rights_error!("user", "The current password is wrong")),
        Err(PasswordError::UserNotFound) => not_found!(sheef_not_found_error!("user", "The user was not found")),
        Err(PasswordError::UnknownError) => internal_server_error!(sheef_unknown_error!("user", "An unknown error occurred")),
    }
}

pub async fn update_profile(body: web::Json<UpdateProfile>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);

    no_content_or_error!(sheef_database::user::update_me(&username, &body.job, &body.gear_level).await)
}

pub async fn get_profile(req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    get_user(web::Path::from(UserPathInfo { username })).await
}

use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use actix_web::http::StatusCode;
use serde::Deserialize;
use sheef_database::user::user_exists;
use sheef_entities::{User, user};
use sheef_entities::authentication::ChangePassword;
use crate::middleware::authenticate_user::AuthenticationState;

#[derive(Deserialize)]
pub struct UserPathInfo {
    pub username: String,
}

macro_rules! prevent_me {
    ($req:ident, $username:expr) => {
        {
            if match $req.extensions().get::<AuthenticationState>() {
                Some(state) => state,
                None => return HttpResponse::new(StatusCode::CONFLICT)
            }.user.username == $username {
                return HttpResponse::new(StatusCode::CONFLICT);
            };
        }
    };
}

pub async fn get_users() -> HttpResponse {
    let data = web::block(move || sheef_database::user::get_users().map(|users| users.map(|u| User {
        username: u.username,
        job: u.job,
        gear_level: u.gear_level,
        is_mod: u.is_mod,
        is_main_group: u.is_main_group,
    }).collect::<Vec<User>>())).await;
    ok_or_not_found!(data)
}

pub async fn get_user(info: web::Path<UserPathInfo>) -> HttpResponse {
    let data = web::block(move || sheef_database::user::get_user(&info.username).map(|u| u.to_web_user())).await;
    ok_or_not_found!(data)
}

pub async fn create_user(user: web::Json<user::User>) -> HttpResponse {
    let data = web::block(move || sheef_database::user::create_user(&user.username, &user.password, user.is_mod, user.is_main_group, &user.gear_level, &user.job, user.is_hidden).map(|u| u.to_web_user())).await;
    if let Ok(Some(user)) = data {
        created_json!(user)
    } else {
        internal_server_error!()
    }
}

pub async fn delete_user(info: web::Path<UserPathInfo>, req: HttpRequest) -> HttpResponse {
    prevent_me!(req, info.username);
    if !user_exists(&info.username) {
        return not_found!();
    }

    let data = web::block(move || sheef_database::user::delete_user(&info.username)).await;
    no_content_or_internal_server_error!(data)
}

pub async fn add_mod_user(info: web::Path<UserPathInfo>, req: HttpRequest) -> HttpResponse {
    prevent_me!(req, info.username);
    if !user_exists(&info.username) {
        return not_found!();
    }

    let data = web::block(move || sheef_database::user::change_mod_status(&info.username, true)).await;
    no_content_or_internal_server_error!(data)
}

pub async fn remove_mod_user(info: web::Path<UserPathInfo>, req: HttpRequest) -> HttpResponse {
    prevent_me!(req, info.username);
    if !user_exists(&info.username) {
        return not_found!();
    }

    let data = web::block(move || sheef_database::user::change_mod_status(&info.username, false)).await;
    no_content_or_internal_server_error!(data)
}

pub async fn add_main_group_user(info: web::Path<UserPathInfo>) -> HttpResponse {
    if !user_exists(&info.username) {
        return not_found!();
    }

    let data = web::block(move || sheef_database::user::change_main_group(&info.username, true)).await;
    no_content_or_internal_server_error!(data)
}

pub async fn remove_main_group_user(info: web::Path<UserPathInfo>) -> HttpResponse {
    if !user_exists(&info.username) {
        return not_found!();
    }

    let data = web::block(move || sheef_database::user::change_main_group(&info.username, false)).await;
    no_content_or_internal_server_error!(data)
}

pub async fn change_password(info: web::Path<UserPathInfo>, body: web::Json<ChangePassword>, req: HttpRequest) -> HttpResponse {
    prevent_me!(req, info.username);
    if !user_exists(&info.username) {
        return not_found!();
    }

    let data =  web::block(move || sheef_database::user::change_password(&info.username.to_string(), &body.new_password)).await;
    no_content_or_internal_server_error!(data)
}
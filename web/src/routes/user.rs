use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use actix_web::http::StatusCode;
use serde::Deserialize;
use sheef_entities::{User, user};
use sheef_entities::authentication::ChangePassword;
use crate::middleware::authenticate_user::AuthenticationState;

#[derive(Deserialize)]
pub struct UserPathInfo {
    username: String,
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
    }))).await;
    if let Ok(Some(users)) = data {
        HttpResponse::Ok().json(web::Json(users.collect::<Vec<User>>()))
    } else {
        HttpResponse::new(StatusCode::NOT_FOUND)
    }
}

pub async fn get_user(info: web::Path<UserPathInfo>) -> HttpResponse {
    let data = web::block(move || match sheef_database::user::get_user(&info.username) {
        Some(u) => Some(u.to_web_user()),
        None => None
    }).await;
    if let Ok(Some(user)) = data {
        HttpResponse::Ok().json(web::Json(user))
    } else {
        HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn create_user(user: web::Json<user::User>) -> HttpResponse {
    let data = web::block(move || match sheef_database::user::create_user(&user.username, &user.password, user.is_mod, user.is_main_group, &user.gear_level, &user.job, user.is_hidden) {
        Some(u) => Some(u.to_web_user()),
        None => None
    }).await;
    if let Ok(Some(user)) = data {
        HttpResponse::Created().json(web::Json(user))
    } else {
        HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn delete_user(info: web::Path<UserPathInfo>, req: HttpRequest) -> HttpResponse {
    prevent_me!(req, info.username);
    if let Ok(Ok(_)) = web::block(move || sheef_database::user::delete_user(&info.username)).await {
        HttpResponse::new(StatusCode::NO_CONTENT)
    } else {
        HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn add_mod_user(info: web::Path<UserPathInfo>, req: HttpRequest) -> HttpResponse {
    prevent_me!(req, info.username);
    if let Ok(Ok(_)) = web::block(move || sheef_database::user::change_mod_status(&info.username, true)).await {
        HttpResponse::new(StatusCode::NO_CONTENT)
    } else {
        HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn remove_mod_user(info: web::Path<UserPathInfo>, req: HttpRequest) -> HttpResponse {
    prevent_me!(req, info.username);
    if let Ok(Ok(_)) = web::block(move || sheef_database::user::change_mod_status(&info.username, false)).await {
        HttpResponse::new(StatusCode::NO_CONTENT)
    } else {
        HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn add_main_group_user(info: web::Path<UserPathInfo>) -> HttpResponse {
    if let Ok(Ok(_)) = web::block(move || sheef_database::user::change_main_group(&info.username, true)).await {
        HttpResponse::new(StatusCode::NO_CONTENT)
    } else {
        HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn remove_main_group_user(info: web::Path<UserPathInfo>) -> HttpResponse {
    if let Ok(Ok(_)) = web::block(move || sheef_database::user::change_main_group(&info.username, false)).await {
        HttpResponse::new(StatusCode::NO_CONTENT)
    } else {
        HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn change_password(info: web::Path<UserPathInfo>, body: web::Json<ChangePassword>, req: HttpRequest) -> HttpResponse {
    prevent_me!(req, info.username);
    if (web::block(move || sheef_database::user::change_password(&info.username.to_string(), &body.new_password)).await).is_ok() {
        HttpResponse::new(StatusCode::NO_CONTENT)
    } else {
        HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
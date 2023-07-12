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

pub async fn get_users() -> HttpResponse {
    let data = web::block(move || sheef_database::user::get_users().map(|users| users.map(|u| User {
        username: u.username,
        job: u.job,
        gear_level: u.gear_level,
        is_mod: u.is_mod,
        is_main_group: u.is_main_group,
    }))).await;
    match data {
        Ok(users) => match users {
            Some(users) => HttpResponse::Ok().json(web::Json(users.collect::<Vec<User>>())),
            None => HttpResponse::new(StatusCode::NOT_FOUND),
        }
        Err(_) => HttpResponse::new(StatusCode::NOT_FOUND)
    }
}

pub async fn get_user(info: web::Path<UserPathInfo>) -> HttpResponse {
    let data = web::block(move || match sheef_database::user::get_user(&info.username) {
        Some(u) => Some(User {
            username: u.username,
            job: u.job,
            gear_level: u.gear_level,
            is_mod: u.is_mod,
            is_main_group: u.is_main_group,
        }),
        None => None
    })
        .await;
    match data {
        Ok(user) => match user {
            Some(user) => HttpResponse::Ok().json(web::Json(user)),
            None => HttpResponse::new(StatusCode::NOT_FOUND),
        }
        Err(_) => HttpResponse::new(StatusCode::NOT_FOUND)
    }
}

pub async fn create_user(user: web::Json<user::User>) -> HttpResponse {
    let data = web::block(move || match sheef_database::user::create_user(&user.username, &user.password, user.is_mod, user.is_main_group, &user.gear_level, &user.job, user.is_hidden) {
        Some(u) => Some(User {
            username: u.username,
            job: u.job,
            gear_level: u.gear_level,
            is_mod: u.is_mod,
            is_main_group: u.is_main_group,
        }),
        None => None
    })
        .await;
    match data {
        Ok(user) => match user {
            Some(user) => HttpResponse::Created().json(web::Json(user)),
            None => HttpResponse::new(StatusCode::NOT_FOUND),
        }
        Err(_) => HttpResponse::new(StatusCode::NOT_FOUND)
    }
}

pub async fn delete_user(info: web::Path<UserPathInfo>, _req: HttpRequest) -> HttpResponse {
    if match _req.extensions().get::<AuthenticationState>() {
        Some(state) => state,
        None => return HttpResponse::new(StatusCode::CONFLICT)
    }.user.username == info.username {
        return HttpResponse::new(StatusCode::CONFLICT);
    };

    match web::block(move || sheef_database::user::delete_user(&info.username)).await {
        Ok(res) => match res {
            Ok(_) => HttpResponse::new(StatusCode::NO_CONTENT),
            Err(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
        Err(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn add_mod_user(info: web::Path<UserPathInfo>, _req: HttpRequest) -> HttpResponse {
    if match _req.extensions().get::<AuthenticationState>() {
        Some(state) => state,
        None => return HttpResponse::new(StatusCode::CONFLICT)
    }.user.username == info.username {
        return HttpResponse::new(StatusCode::CONFLICT);
    };

    match web::block(move || sheef_database::user::change_mod_status(&info.username, true)).await {
        Ok(res) => match res {
            Ok(_) => HttpResponse::new(StatusCode::NO_CONTENT),
            Err(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
        Err(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn remove_mod_user(info: web::Path<UserPathInfo>, _req: HttpRequest) -> HttpResponse {
    if match _req.extensions().get::<AuthenticationState>() {
        Some(state) => state,
        None => return HttpResponse::new(StatusCode::CONFLICT)
    }.user.username == info.username {
        return HttpResponse::new(StatusCode::CONFLICT);
    };

    match web::block(move || sheef_database::user::change_mod_status(&info.username, false)).await {
        Ok(res) => match res {
            Ok(_) => HttpResponse::new(StatusCode::NO_CONTENT),
            Err(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
        Err(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn add_main_group_user(info: web::Path<UserPathInfo>) -> HttpResponse {
    match web::block(move || sheef_database::user::change_main_group(&info.username, true)).await {
        Ok(res) => match res {
            Ok(_) => HttpResponse::new(StatusCode::NO_CONTENT),
            Err(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
        Err(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn remove_main_group_user(info: web::Path<UserPathInfo>) -> HttpResponse {
    match web::block(move || sheef_database::user::change_main_group(&info.username, false)).await {
        Ok(res) => match res {
            Ok(_) => HttpResponse::new(StatusCode::NO_CONTENT),
            Err(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
        Err(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn change_password(info: web::Path<UserPathInfo>, body: web::Json<ChangePassword>, _req: HttpRequest) -> HttpResponse {
    if match _req.extensions().get::<AuthenticationState>() {
        Some(state) => state,
        None => return HttpResponse::new(StatusCode::CONFLICT)
    }.user.username == info.username {
        return HttpResponse::new(StatusCode::CONFLICT);
    };

    match sheef_database::user::change_password(&info.username.to_string(), &body.new_password) {
        Ok(_) => HttpResponse::new(StatusCode::NO_CONTENT),
        Err(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
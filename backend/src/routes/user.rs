use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;
use sheef_database::user::{PasswordError, user_exists};
use sheef_entities::{UpdateProfile, User, user};
use sheef_entities::authentication::{ChangeMyPassword, ChangePassword};

#[derive(Deserialize)]
pub struct UserPathInfo {
    pub username: String,
}

macro_rules! prevent_me {
    ($req:ident, $username:expr) => {
        {
            if username!($req) == $username {
                return conflict!();
            };
        }
    };
}

pub async fn get_users() -> HttpResponse {
    let data = sheef_database::user::get_users().await.map(|users| users.into_iter().map(|u| User {
        username: u.username,
        job: u.job,
        gear_level: u.gear_level,
        is_mod: u.is_mod,
        is_main_group: u.is_main_group,
    }).collect::<Vec<User>>());
    ok_or_not_found!(data)
}

pub async fn get_user(info: web::Path<UserPathInfo>) -> HttpResponse {
    let data = sheef_database::user::get_user(&info.username).await.map(|u| u.to_web_user());
    ok_or_not_found!(data)
}

pub async fn create_user(user: web::Json<user::User>) -> HttpResponse {
    if user_exists(&user.username).await {
        return conflict!();
    }

    let data = sheef_database::user::create_user(&user.username, &user.password, user.is_mod, user.is_main_group, &user.gear_level, &user.job, user.is_hidden).await.map(|u| u.to_web_user());
    if let Some(user) = data {
        created_json!(user)
    } else {
        internal_server_error!()
    }
}

pub async fn delete_user(info: web::Path<UserPathInfo>, req: HttpRequest) -> HttpResponse {
    prevent_me!(req, info.username);
    if !user_exists(&info.username).await {
        return not_found!();
    }

    let data = sheef_database::user::delete_user(&info.username).await;
    no_content_or_internal_server_error!(data)
}

pub async fn add_mod_user(info: web::Path<UserPathInfo>, req: HttpRequest) -> HttpResponse {
    prevent_me!(req, info.username);
    if !user_exists(&info.username).await {
        return not_found!();
    }

    let data = sheef_database::user::change_mod_status(&info.username, true).await;
    no_content_or_internal_server_error!(data)
}

pub async fn remove_mod_user(info: web::Path<UserPathInfo>, req: HttpRequest) -> HttpResponse {
    prevent_me!(req, info.username);
    if !user_exists(&info.username).await {
        return not_found!();
    }

    let data = sheef_database::user::change_mod_status(&info.username, false).await;
    no_content_or_internal_server_error!(data)
}

pub async fn add_main_group_user(info: web::Path<UserPathInfo>) -> HttpResponse {
    if !user_exists(&info.username).await {
        return not_found!();
    }

    let data = sheef_database::user::change_main_group(&info.username, true).await;
    no_content_or_internal_server_error!(data)
}

pub async fn remove_main_group_user(info: web::Path<UserPathInfo>) -> HttpResponse {
    if !user_exists(&info.username).await {
        return not_found!();
    }

    let data = sheef_database::user::change_main_group(&info.username, false).await;
    no_content_or_internal_server_error!(data)
}

pub async fn change_password(info: web::Path<UserPathInfo>, body: web::Json<ChangePassword>, req: HttpRequest) -> HttpResponse {
    prevent_me!(req, info.username);
    if !user_exists(&info.username).await {
        return not_found!();
    }

    let data = sheef_database::user::change_password(&info.username.to_string(), &body.new_password).await;
    no_content_or_internal_server_error!(data)
}

pub async fn change_my_password(body: web::Json<ChangeMyPassword>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    match sheef_database::user::change_my_password(&username, &body.old_password, &body.new_password).await {
        Ok(_) => no_content!(),
        Err(PasswordError::WrongPassword) => forbidden!(),
        Err(PasswordError::UserNotFound) => not_found!(),
        Err(PasswordError::UnknownError) => internal_server_error!(),
    }
}

pub async fn update_profile(body: web::Json<UpdateProfile>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    if let Ok(_) = sheef_database::user::update_me(&username, &body.job, &body.gear_level).await {
        no_content!()
    } else {
        internal_server_error!()
    }
}

pub async fn get_profile(req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    get_user(web::Path::from(UserPathInfo { username })).await
}

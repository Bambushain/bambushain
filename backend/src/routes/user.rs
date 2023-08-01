use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;

use pandaparty_dbal::prelude::*;
use pandaparty_entities::prelude::*;

use crate::sse::NotificationState;

#[derive(Deserialize)]
pub struct UserPathInfo {
    pub username: String,
}

macro_rules! prevent_me {
    ($req:ident, $username:expr, $error_message:expr) => {
        {
            if username!($req) == $username {
                return conflict!(pandaparty_validation_error!("user", $error_message));
            };
        }
    };
}

pub async fn get_users() -> HttpResponse {
    ok_or_error!(pandaparty_dbal::user::get_users().await.map(|users| users.into_iter().map(|u| u.to_web_user()).collect::<Vec<WebUser>>()))
}

pub async fn get_user(info: web::Path<UserPathInfo>) -> HttpResponse {
    ok_or_error!(pandaparty_dbal::user::get_user(info.username.clone()).await.map(|u| u.to_web_user()))
}

pub async fn create_user(user: web::Json<User>, notification_state: web::Data<NotificationState>) -> HttpResponse {
    if user_exists(user.username.clone()).await {
        return conflict!(pandaparty_exists_already_error!("user", "A user with the name already exists"));
    }

    let data = pandaparty_dbal::user::create_user(user.into_inner()).await.map(|u| u.to_web_user());
    actix_web::rt::spawn(async move {
        notification_state.crew_broadcaster.notify_change().await;
    });

    created_or_error!(data)
}

pub async fn delete_user(info: web::Path<UserPathInfo>, notification_state: web::Data<NotificationState>, req: HttpRequest) -> HttpResponse {
    prevent_me!(req, info.username, "You cannot delete yourself");
    if !user_exists(info.username.clone()).await {
        return not_found!(pandaparty_not_found_error!("user", "The user was not found"));
    }

    let data = pandaparty_dbal::user::delete_user(info.username.clone()).await;
    actix_web::rt::spawn(async move {
        notification_state.crew_broadcaster.notify_change().await;
    });

    no_content_or_error!(data)
}

pub async fn add_mod_user(info: web::Path<UserPathInfo>, notification_state: web::Data<NotificationState>, req: HttpRequest) -> HttpResponse {
    prevent_me!(req, info.username, "You cannot make yourself mod");
    if !user_exists(info.username.clone()).await {
        return not_found!(pandaparty_not_found_error!("user", "The user was not found"));
    }

    let data = change_mod_status(info.username.clone(), true).await;
    actix_web::rt::spawn(async move {
        notification_state.crew_broadcaster.notify_change().await;
    });

    no_content_or_error!(data)
}

pub async fn remove_mod_user(info: web::Path<UserPathInfo>, notification_state: web::Data<NotificationState>, req: HttpRequest) -> HttpResponse {
    prevent_me!(req, info.username, "You cannot revoke your own mod rights");
    if !user_exists(info.username.clone()).await {
        return not_found!(pandaparty_not_found_error!("user", "The user was not found"));
    }

    let data = change_mod_status(info.username.clone(), false).await;
    actix_web::rt::spawn(async move {
        notification_state.crew_broadcaster.notify_change().await;
    });

    no_content_or_error!(data)
}

pub async fn change_password(info: web::Path<UserPathInfo>, body: web::Json<ChangePassword>, req: HttpRequest) -> HttpResponse {
    prevent_me!(req, info.username, "You cannot change your own password using this endpoint");
    if !user_exists(info.username.clone()).await {
        return not_found!(pandaparty_not_found_error!("user", "The user was not found"));
    }

    no_content_or_error!(pandaparty_dbal::user::change_password(info.username.clone(), body.new_password.clone()).await)
}

pub async fn change_my_password(body: web::Json<ChangeMyPassword>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    match pandaparty_dbal::user::change_my_password(username.clone(), body.old_password.clone(), body.new_password.clone()).await {
        Ok(_) => no_content!(),
        Err(PasswordError::WrongPassword) => forbidden!(pandaparty_insufficient_rights_error!("user", "The current password is wrong")),
        Err(PasswordError::UserNotFound) => not_found!(pandaparty_not_found_error!("user", "The user was not found")),
        Err(PasswordError::UnknownError) => internal_server_error!(pandaparty_unknown_error!("user", "An unknown error occurred")),
    }
}

pub async fn update_profile(body: web::Json<UpdateProfile>, notification_state: web::Data<NotificationState>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);

    let data = update_me(username, body.job.clone(), body.gear_level.clone()).await;
    actix_web::rt::spawn(async move {
        notification_state.crew_broadcaster.notify_change().await;
    });

    no_content_or_error!(data)
}

pub async fn update_user_profile(info: web::Path<UserPathInfo>, body: web::Json<UpdateProfile>, notification_state: web::Data<NotificationState>) -> HttpResponse {
    let data = update_me(info.username.clone(), body.job.clone(), body.gear_level.clone()).await;
    actix_web::rt::spawn(async move {
        notification_state.crew_broadcaster.notify_change().await;
    });

    no_content_or_error!(data)
}

pub async fn get_profile(req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    get_user(web::Path::from(UserPathInfo { username })).await
}

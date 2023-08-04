use actix_web::{HttpResponse, web};
use serde::Deserialize;

use pandaparty_dbal::prelude::*;
use pandaparty_entities::prelude::*;

use crate::DbConnection;
use crate::middleware::authenticate_user::Authentication;
use crate::sse::NotificationState;

#[derive(Deserialize)]
pub struct UserPathInfo {
    pub username: String,
}

macro_rules! prevent_me {
    ($me:expr, $username:expr, $error_message:expr) => {
        {
            if $me == $username {
                return conflict!(pandaparty_validation_error!("user", $error_message));
            };
        }
    };
}

pub async fn get_users(db: DbConnection) -> HttpResponse {
    ok_or_error!(pandaparty_dbal::user::get_users(&db).await.map(|users| users.into_iter().map(|u| u.to_web_user()).collect::<Vec<WebUser>>()))
}

pub async fn get_user(info: web::Path<UserPathInfo>, db: DbConnection) -> HttpResponse {
    ok_or_error!(pandaparty_dbal::user::get_user(info.username.clone(),&db).await.map(|u| u.to_web_user()))
}

pub async fn create_user(user: web::Json<User>, notification_state: web::Data<NotificationState>, db: DbConnection) -> HttpResponse {
    if user_exists(user.username.clone(), &db).await {
        return conflict!(pandaparty_exists_already_error!("user", "A user with the name already exists"));
    }

    let data = pandaparty_dbal::user::create_user(user.into_inner(), &db).await.map(|u| u.to_web_user());
    actix_web::rt::spawn(async move {
        notification_state.crew_broadcaster.notify_change().await;
    });

    created_or_error!(data)
}

pub async fn delete_user(info: web::Path<UserPathInfo>, notification_state: web::Data<NotificationState>, authentication_state: Authentication, db: DbConnection) -> HttpResponse {
    prevent_me!(authentication_state.user.username.clone(), info.username, "You cannot delete yourself");
    if !user_exists(info.username.clone(), &db).await {
        return not_found!(pandaparty_not_found_error!("user", "The user was not found"));
    }

    let data = pandaparty_dbal::user::delete_user(info.username.clone(), &db).await;
    actix_web::rt::spawn(async move {
        notification_state.crew_broadcaster.notify_change().await;
    });

    no_content_or_error!(data)
}

pub async fn add_mod_user(info: web::Path<UserPathInfo>, notification_state: web::Data<NotificationState>, authentication_state: Authentication, db: DbConnection) -> HttpResponse {
    prevent_me!(authentication_state.user.username.clone(),  info.username, "You cannot make yourself mod");
    if !user_exists(info.username.clone(), &db).await {
        return not_found!(pandaparty_not_found_error!("user", "The user was not found"));
    }

    let data = change_mod_status(info.username.clone(), true, &db).await;
    actix_web::rt::spawn(async move {
        notification_state.crew_broadcaster.notify_change().await;
    });

    no_content_or_error!(data)
}

pub async fn remove_mod_user(info: web::Path<UserPathInfo>, notification_state: web::Data<NotificationState>, authentication_state: Authentication, db: DbConnection) -> HttpResponse {
    prevent_me!(authentication_state.user.username.clone(), info.username, "You cannot revoke your own mod rights");
    if !user_exists(info.username.clone(), &db).await {
        return not_found!(pandaparty_not_found_error!("user", "The user was not found"));
    }

    let data = change_mod_status(info.username.clone(), false, &db).await;
    actix_web::rt::spawn(async move {
        notification_state.crew_broadcaster.notify_change().await;
    });

    no_content_or_error!(data)
}

pub async fn change_password(info: web::Path<UserPathInfo>, body: web::Json<ChangePassword>, authentication_state: Authentication, db: DbConnection) -> HttpResponse {
    prevent_me!(authentication_state.user.username.clone(), info.username, "You cannot change your own password using this endpoint");
    if !user_exists(info.username.clone(), &db).await {
        return not_found!(pandaparty_not_found_error!("user", "The user was not found"));
    }

    no_content_or_error!(pandaparty_dbal::user::change_password(info.username.clone(), body.new_password.clone(),&db).await)
}

pub async fn change_my_password(body: web::Json<ChangeMyPassword>, authentication_state: Authentication, db: DbConnection) -> HttpResponse {
    match pandaparty_dbal::user::change_my_password(authentication_state.user.username.clone(), body.old_password.clone(), body.new_password.clone(), &db).await {
        Ok(_) => no_content!(),
        Err(PasswordError::WrongPassword) => forbidden!(pandaparty_insufficient_rights_error!("user", "The current password is wrong")),
        Err(PasswordError::UserNotFound) => not_found!(pandaparty_not_found_error!("user", "The user was not found")),
        Err(PasswordError::UnknownError) => internal_server_error!(pandaparty_unknown_error!("user", "An unknown error occurred")),
    }
}

pub async fn update_profile(body: web::Json<UpdateProfile>, notification_state: web::Data<NotificationState>, authentication_state: Authentication, db: DbConnection) -> HttpResponse {
    let data = update_me(authentication_state.user.username.clone(), body.job.clone(), body.gear_level.clone(), &db).await;
    actix_web::rt::spawn(async move {
        notification_state.crew_broadcaster.notify_change().await;
    });

    no_content_or_error!(data)
}

pub async fn update_user_profile(info: web::Path<UserPathInfo>, body: web::Json<UpdateProfile>, notification_state: web::Data<NotificationState>, db: DbConnection) -> HttpResponse {
    let data = update_me(info.username.clone(), body.job.clone(), body.gear_level.clone(), &db).await;
    actix_web::rt::spawn(async move {
        notification_state.crew_broadcaster.notify_change().await;
    });

    no_content_or_error!(data)
}

pub async fn get_profile(authentication_state: Authentication) -> HttpResponse {
    ok_json!(authentication_state.user.to_web_user())
}

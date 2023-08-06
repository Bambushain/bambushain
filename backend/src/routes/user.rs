use actix_web::{HttpResponse, web};
use serde::Deserialize;

use pandaparty_dbal::prelude::*;
use pandaparty_entities::prelude::*;

use crate::DbConnection;
use crate::middleware::authenticate_user::Authentication;
use crate::sse::Notification;

#[derive(Deserialize)]
pub struct UserPathInfo {
    pub id: i32,
}

macro_rules! prevent_me {
    ($me:expr, $passed_user:expr, $error_message:expr) => {
        {
            if $me == $passed_user {
                return conflict!(pandaparty_validation_error!("user", $error_message));
            };
        }
    };
}

pub async fn get_users(db: DbConnection) -> HttpResponse {
    ok_or_error!(pandaparty_dbal::user::get_users(&db).await.map(|users| users.into_iter().map(|u| u.to_web_user()).collect::<Vec<WebUser>>()))
}

pub async fn get_user(info: web::Path<UserPathInfo>, db: DbConnection) -> HttpResponse {
    ok_or_error!(pandaparty_dbal::user::get_user(info.id,&db).await.map(|u| u.to_web_user()))
}

pub async fn create_user(user: web::Json<User>, notification: Notification, db: DbConnection) -> HttpResponse {
    if user_exists(user.id, &db).await {
        return conflict!(pandaparty_exists_already_error!("user", "A user with the name already exists"));
    }

    let data = pandaparty_dbal::user::create_user(user.into_inner(), &db).await.map(|u| u.to_web_user());
    if data.is_ok() {
        actix_web::rt::spawn(async move {
            notification.user_broadcaster.notify_change().await;
        });
    }

    created_or_error!(data)
}

pub async fn delete_user(info: web::Path<UserPathInfo>, notification: Notification, authentication: Authentication, db: DbConnection) -> HttpResponse {
    prevent_me!(authentication.user.id, info.id, "You cannot delete yourself");
    if !user_exists(info.id, &db).await {
        return not_found!(pandaparty_not_found_error!("user", "The user was not found"));
    }

    let data = pandaparty_dbal::user::delete_user(info.id, &db).await;
    if data.is_ok() {
        actix_web::rt::spawn(async move {
            notification.user_broadcaster.notify_change().await;
        });
    }

    no_content_or_error!(data)
}

pub async fn add_mod_user(info: web::Path<UserPathInfo>, notification: Notification, authentication: Authentication, db: DbConnection) -> HttpResponse {
    prevent_me!(authentication.user.id,  info.id, "You cannot make yourself mod");
    if !user_exists(info.id, &db).await {
        return not_found!(pandaparty_not_found_error!("user", "The user was not found"));
    }

    let data = change_mod_status(info.id, true, &db).await;
    if data.is_ok() {
        actix_web::rt::spawn(async move {
            notification.user_broadcaster.notify_change().await;
        });
    }

    no_content_or_error!(data)
}

pub async fn remove_mod_user(info: web::Path<UserPathInfo>, notification: Notification, authentication: Authentication, db: DbConnection) -> HttpResponse {
    prevent_me!(authentication.user.id, info.id, "You cannot revoke your own mod rights");
    if !user_exists(info.id, &db).await {
        return not_found!(pandaparty_not_found_error!("user", "The user was not found"));
    }

    let data = change_mod_status(info.id, false, &db).await;
    actix_web::rt::spawn(async move {
        notification.user_broadcaster.notify_change().await;
    });

    no_content_or_error!(data)
}

pub async fn change_password(info: web::Path<UserPathInfo>, body: web::Json<ChangePassword>, authentication: Authentication, db: DbConnection) -> HttpResponse {
    prevent_me!(authentication.user.id, info.id, "You cannot change your own password using this endpoint");
    if !user_exists(info.id, &db).await {
        return not_found!(pandaparty_not_found_error!("user", "The user was not found"));
    }

    no_content_or_error!(pandaparty_dbal::user::change_password(info.id, body.new_password.clone(),&db).await)
}

pub async fn change_my_password(body: web::Json<ChangeMyPassword>, authentication: Authentication, db: DbConnection) -> HttpResponse {
    match pandaparty_dbal::user::change_my_password(authentication.user.id, body.old_password.clone(), body.new_password.clone(), &db).await {
        Ok(_) => no_content!(),
        Err(PasswordError::WrongPassword) => forbidden!(pandaparty_insufficient_rights_error!("user", "The current password is wrong")),
        Err(PasswordError::UserNotFound) => not_found!(pandaparty_not_found_error!("user", "The user was not found")),
        Err(PasswordError::UnknownError) => internal_server_error!(pandaparty_unknown_error!("user", "An unknown error occurred")),
    }
}

pub async fn update_profile(body: web::Json<UpdateProfile>, notification: Notification, authentication: Authentication, db: DbConnection) -> HttpResponse {
    let data = update_me(authentication.user.id, body.job.clone(), body.gear_level.clone(), body.discord_name.clone(), &db).await;
    if data.is_ok() {
        actix_web::rt::spawn(async move {
            notification.user_broadcaster.notify_change().await;
        });
    }

    no_content_or_error!(data)
}

pub async fn update_user_profile(info: web::Path<UserPathInfo>, body: web::Json<UpdateProfile>, notification: Notification, db: DbConnection) -> HttpResponse {
    let data = update_me(info.id, body.job.clone(), body.gear_level.clone(), body.discord_name.clone(), &db).await;
    if data.is_ok() {
        actix_web::rt::spawn(async move {
            notification.user_broadcaster.notify_change().await;
        });
    }

    no_content_or_error!(data)
}

pub async fn get_profile(authentication: Authentication) -> HttpResponse {
    ok_json!(authentication.user.to_web_user())
}

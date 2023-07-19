use std::collections::BTreeMap;

use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;

use sheef_database::savage_mount::savage_mount_exists;
use sheef_database::user::user_exists;
use sheef_entities::{SavageMount, sheef_exists_already_error, sheef_not_found_error, SheefError};

use crate::sse::NotificationState;

#[derive(Deserialize)]
pub struct SavageMountUsernamePathInfo {
    pub savage_mount: String,
    pub username: String,
}

#[derive(Deserialize)]
pub struct SavageMountPathInfo {
    pub savage_mount: String,
}

pub async fn get_savage_mounts() -> HttpResponse {
    let savage_mounts = match sheef_database::savage_mount::get_savage_mounts().await {
        Ok(savage_mounts) => savage_mounts,
        Err(err) => return ok_or_error!(Err::<(), SheefError>(err))
    };
    let mut response = BTreeMap::new();
    for savage_mount in savage_mounts {
        response.insert(savage_mount.to_string(), vec![]);
        let mut users_for_savage_mount = sheef_database::savage_mount::get_users_for_savage_mount(&savage_mount).await.expect("Savage mount does exist");
        response.get_mut(&savage_mount).expect("Vector should exist").append(&mut users_for_savage_mount);
    }

    ok_or_error!(Ok::<BTreeMap<String, Vec<String>>, SheefError>(response))
}

pub async fn activate_savage_mount_for_user(path: web::Path<SavageMountUsernamePathInfo>, notification_state: web::Data<NotificationState>) -> HttpResponse {
    if !user_exists(&path.username).await {
        return not_found!(sheef_not_found_error!("user", "User not found"));
    }

    if !savage_mount_exists(&path.savage_mount).await {
        return not_found!(sheef_not_found_error!("savage-mount", "Savage mount not found"));
    }

    let data = sheef_database::savage_mount::activate_savage_mount_for_user(&path.savage_mount, &path.username).await;
    actix_web::rt::spawn(async move {
        notification_state.savage_mount_broadcaster.notify_change().await;
    });

    no_content_or_error!(data)
}

pub async fn activate_savage_mount_for_me(path: web::Path<SavageMountPathInfo>, notification_state: web::Data<NotificationState>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    activate_savage_mount_for_user(web::Path::<SavageMountUsernamePathInfo>::from(SavageMountUsernamePathInfo { username, savage_mount: path.savage_mount.to_string() }), notification_state).await
}

pub async fn deactivate_savage_mount_for_user(path: web::Path<SavageMountUsernamePathInfo>, notification_state: web::Data<NotificationState>) -> HttpResponse {
    if !user_exists(&path.username).await {
        return not_found!(sheef_not_found_error!("user", "User not found"));
    }

    if !savage_mount_exists(&path.savage_mount).await {
        return not_found!(sheef_not_found_error!("savage-mount", "Savage mount not found"));
    }

    let data = sheef_database::savage_mount::deactivate_savage_mount_for_user(&path.savage_mount, &path.username).await;
    actix_web::rt::spawn(async move {
        notification_state.savage_mount_broadcaster.notify_change().await;
    });

    no_content_or_error!(data)
}

pub async fn deactivate_savage_mount_for_me(path: web::Path<SavageMountPathInfo>, notification_state: web::Data<NotificationState>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    deactivate_savage_mount_for_user(web::Path::<SavageMountUsernamePathInfo>::from(SavageMountUsernamePathInfo { username, savage_mount: path.savage_mount.to_string() }), notification_state).await
}

pub async fn delete_savage_mount(path: web::Path<SavageMountPathInfo>, notification_state: web::Data<NotificationState>) -> HttpResponse {
    if !savage_mount_exists(&path.savage_mount).await {
        return not_found!(sheef_not_found_error!("savage-mount", "Savage mount not found"));
    }

    let data = sheef_database::savage_mount::delete_savage_mount(&path.savage_mount).await;
    actix_web::rt::spawn(async move {
        notification_state.savage_mount_broadcaster.notify_change().await;
    });

    no_content_or_error!(data)
}

pub async fn create_savage_mount(body: web::Json<SavageMount>, notification_state: web::Data<NotificationState>) -> HttpResponse {
    let savage_mount = body.name.to_string();
    if savage_mount_exists(&body.name).await {
        return conflict!(sheef_exists_already_error!("savage-mount", "Savage mount already exists"));
    }

    let data = sheef_database::savage_mount::create_savage_mount(&body.name).await.map(|_| SavageMount { name: savage_mount });
    actix_web::rt::spawn(async move {
        notification_state.savage_mount_broadcaster.notify_change().await;
    });

    created_or_error!(data)
}

pub async fn update_savage_mount(path: web::Path<SavageMountPathInfo>, body: web::Json<SavageMount>, notification_state: web::Data<NotificationState>) -> HttpResponse {
    if !savage_mount_exists(&path.savage_mount).await {
        return not_found!(sheef_not_found_error!("savage-mount", "Savage mount not found"));
    }

    if savage_mount_exists(&body.name).await && body.name != path.savage_mount {
        return conflict!(sheef_exists_already_error!("savage-mount", "Savage mount already exists"));
    }

    let data = sheef_database::savage_mount::update_savage_mount(&path.savage_mount, &body.name).await;
    actix_web::rt::spawn(async move {
        notification_state.savage_mount_broadcaster.notify_change().await;
    });

    no_content_or_error!(data)
}
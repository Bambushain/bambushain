use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;

use sheef_dbal::prelude::*;
use sheef_entities::prelude::*;

use crate::sse::NotificationState;

#[derive(Deserialize)]
pub struct MountUsernamePathInfo {
    pub mount: String,
    pub username: String,
}

#[derive(Deserialize)]
pub struct MountPathInfo {
    pub mount: String,
}

pub async fn get_mounts() -> HttpResponse {
    ok_or_error!(sheef_dbal::mount::get_mounts().await)
}

pub async fn activate_mount_for_user(path: web::Path<MountUsernamePathInfo>, notification_state: web::Data<NotificationState>) -> HttpResponse {
    if !user_exists(path.username.clone()).await {
        return not_found!(sheef_not_found_error!("user", "User not found"));
    }

    if !mount_exists(path.mount.clone()).await {
        return not_found!(sheef_not_found_error!("mount", "Mount not found"));
    }

    let data = sheef_dbal::mount::activate_mount_for_user(path.mount.clone(), path.username.clone()).await;
    actix_web::rt::spawn(async move {
        notification_state.mount_broadcaster.notify_change().await;
    });

    no_content_or_error!(data)
}

pub async fn activate_mount_for_me(path: web::Path<MountPathInfo>, notification_state: web::Data<NotificationState>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    activate_mount_for_user(web::Path::<MountUsernamePathInfo>::from(MountUsernamePathInfo { username, mount: path.mount.to_string() }), notification_state).await
}

pub async fn deactivate_mount_for_user(path: web::Path<MountUsernamePathInfo>, notification_state: web::Data<NotificationState>) -> HttpResponse {
    if !user_exists(path.username.clone()).await {
        return not_found!(sheef_not_found_error!("user", "User not found"));
    }

    if !mount_exists(path.mount.clone()).await {
        return not_found!(sheef_not_found_error!("mount", "Mount not found"));
    }

    let data = sheef_dbal::mount::deactivate_mount_for_user(path.mount.clone(), path.username.clone()).await;
    actix_web::rt::spawn(async move {
        notification_state.mount_broadcaster.notify_change().await;
    });

    no_content_or_error!(data)
}

pub async fn deactivate_mount_for_me(path: web::Path<MountPathInfo>, notification_state: web::Data<NotificationState>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    deactivate_mount_for_user(web::Path::<MountUsernamePathInfo>::from(MountUsernamePathInfo { username, mount: path.mount.to_string() }), notification_state).await
}

pub async fn delete_mount(path: web::Path<MountPathInfo>, notification_state: web::Data<NotificationState>) -> HttpResponse {
    if !mount_exists(path.mount.clone()).await {
        return not_found!(sheef_not_found_error!("mount", "Mount not found"));
    }

    let data = sheef_dbal::mount::delete_mount(path.mount.clone()).await;
    actix_web::rt::spawn(async move {
        notification_state.mount_broadcaster.notify_change().await;
    });

    no_content_or_error!(data)
}

pub async fn create_mount(body: web::Json<Mount>, notification_state: web::Data<NotificationState>) -> HttpResponse {
    if mount_exists(body.name.clone()).await {
        return conflict!(sheef_exists_already_error!("mount", "Mount already exists"));
    }

    let data = sheef_dbal::mount::create_mount(body.into_inner()).await.map(|mount| mount);
    actix_web::rt::spawn(async move {
        notification_state.mount_broadcaster.notify_change().await;
    });

    created_or_error!(data)
}

pub async fn update_mount(path: web::Path<MountPathInfo>, body: web::Json<Mount>, notification_state: web::Data<NotificationState>) -> HttpResponse {
    if !mount_exists(path.mount.clone()).await {
        return not_found!(sheef_not_found_error!("mount", "Mount not found"));
    }

    if mount_exists(body.name.clone()).await && body.name != path.mount {
        return conflict!(sheef_exists_already_error!("mount", "Mount already exists"));
    }

    let data = sheef_dbal::mount::update_mount(path.mount.clone(), body.name.clone()).await;
    actix_web::rt::spawn(async move {
        notification_state.mount_broadcaster.notify_change().await;
    });

    no_content_or_error!(data)
}

use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;

use pandaparty_dbal::prelude::*;
use pandaparty_entities::prelude::*;

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
    ok_or_error!(pandaparty_dbal::savage_mount::get_savage_mounts().await)
}

pub async fn activate_savage_mount_for_user(path: web::Path<SavageMountUsernamePathInfo>, notification_state: web::Data<NotificationState>) -> HttpResponse {
    if !user_exists(path.username.clone()).await {
        return not_found!(pandaparty_not_found_error!("user", "User not found"));
    }

    if !savage_mount_exists(path.savage_mount.clone()).await {
        return not_found!(pandaparty_not_found_error!("savage-mount", "Savage mount not found"));
    }

    let data = pandaparty_dbal::savage_mount::activate_savage_mount_for_user(path.savage_mount.clone(), path.username.clone()).await;
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
    if !user_exists(path.username.clone()).await {
        return not_found!(pandaparty_not_found_error!("user", "User not found"));
    }

    if !savage_mount_exists(path.savage_mount.clone()).await {
        return not_found!(pandaparty_not_found_error!("savage-mount", "Savage mount not found"));
    }

    let data = pandaparty_dbal::savage_mount::deactivate_savage_mount_for_user(path.savage_mount.clone(), path.username.clone()).await;
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
    if !savage_mount_exists(path.savage_mount.clone()).await {
        return not_found!(pandaparty_not_found_error!("savage-mount", "Savage mount not found"));
    }

    let data = pandaparty_dbal::savage_mount::delete_savage_mount(path.savage_mount.clone()).await;
    actix_web::rt::spawn(async move {
        notification_state.savage_mount_broadcaster.notify_change().await;
    });

    no_content_or_error!(data)
}

pub async fn create_savage_mount(body: web::Json<SavageMount>, notification_state: web::Data<NotificationState>) -> HttpResponse {
    if savage_mount_exists(body.name.clone()).await {
        return conflict!(pandaparty_exists_already_error!("savage-mount", "Savage mount already exists"));
    }

    let data = pandaparty_dbal::savage_mount::create_savage_mount(body.into_inner()).await.map(|savage_mount| savage_mount);
    actix_web::rt::spawn(async move {
        notification_state.savage_mount_broadcaster.notify_change().await;
    });

    created_or_error!(data)
}

pub async fn update_savage_mount(path: web::Path<SavageMountPathInfo>, body: web::Json<SavageMount>, notification_state: web::Data<NotificationState>) -> HttpResponse {
    if !savage_mount_exists(path.savage_mount.clone()).await {
        return not_found!(pandaparty_not_found_error!("savage-mount", "Savage mount not found"));
    }

    if savage_mount_exists(body.name.clone()).await && body.name != path.savage_mount {
        return conflict!(pandaparty_exists_already_error!("savage-mount", "Savage mount already exists"));
    }

    let data = pandaparty_dbal::savage_mount::update_savage_mount(path.savage_mount.clone(), body.name.clone()).await;
    actix_web::rt::spawn(async move {
        notification_state.savage_mount_broadcaster.notify_change().await;
    });

    no_content_or_error!(data)
}

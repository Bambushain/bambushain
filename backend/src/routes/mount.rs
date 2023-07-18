use std::collections::BTreeMap;
use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;
use sheef_database::mount::mount_exists;
use sheef_database::user::user_exists;
use sheef_entities::Mount;

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
    let mounts = sheef_database::mount::get_mounts().await;
    let mut response = BTreeMap::new();
    for mount in mounts {
        response.insert(mount.to_string(), vec![]);
        let mut users_for_mount = sheef_database::mount::get_users_for_mount(&mount).await.expect("Mount does exist");
        response.get_mut(&mount).expect("Vector should exist").append(&mut users_for_mount);
    }

    if response.is_empty() {
        no_content!()
    } else {
        ok_json!(response)
    }
}

pub async fn activate_mount_for_user(path: web::Path<MountUsernamePathInfo>) -> HttpResponse {
    if !mount_exists(&path.mount).await || !user_exists(&path.username).await {
        return not_found!();
    }

    let data = sheef_database::mount::activate_mount_for_user(&path.mount, &path.username).await;
    no_content_or_internal_server_error!(data)
}

pub async fn activate_mount_for_me(path: web::Path<MountPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    activate_mount_for_user(web::Path::<MountUsernamePathInfo>::from(MountUsernamePathInfo { username, mount: path.mount.to_string() })).await
}

pub async fn deactivate_mount_for_user(path: web::Path<MountUsernamePathInfo>) -> HttpResponse {
    if !mount_exists(&path.mount).await || !user_exists(&path.username).await {
        return not_found!();
    }

    let data = sheef_database::mount::deactivate_mount_for_user(&path.mount, &path.username).await;
    no_content_or_internal_server_error!(data)
}

pub async fn deactivate_mount_for_me(path: web::Path<MountPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    deactivate_mount_for_user(web::Path::<MountUsernamePathInfo>::from(MountUsernamePathInfo { username, mount: path.mount.to_string() })).await
}

pub async fn delete_mount(path: web::Path<MountPathInfo>) -> HttpResponse {
    if !mount_exists(&path.mount).await {
        return not_found!();
    }

    let data = sheef_database::mount::delete_mount(&path.mount).await;
    no_content_or_internal_server_error!(data)
}

pub async fn create_mount(body: web::Json<Mount>) -> HttpResponse {
    let mount = body.name.to_string();
    if mount_exists(&body.name).await {
        return conflict!();
    }

    let data = sheef_database::mount::create_mount(&body.name).await;
    if data.is_ok() {
        created_json!(Mount { name: mount })
    } else {
        internal_server_error!()
    }
}

pub async fn update_mount(path: web::Path<MountPathInfo>, body: web::Json<Mount>) -> HttpResponse {
    if !mount_exists(&path.mount).await {
        return not_found!();
    }

    let data = sheef_database::mount::update_mount(&path.mount, &body.name).await;
    no_content_or_internal_server_error!(data)
}
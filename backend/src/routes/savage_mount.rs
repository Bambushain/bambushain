use std::collections::BTreeMap;
use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;
use sheef_database::savage_mount::savage_mount_exists;
use sheef_database::user::user_exists;
use sheef_entities::SavageMount;
use crate::routes::user::UserPathInfo;

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
    let savage_mounts = sheef_database::savage_mount::get_savage_mounts().await;
    let mut response = BTreeMap::new();
    for savage_mount in savage_mounts {
        response.insert(savage_mount.to_string(), vec![]);
        let mut users_for_savage_mount = sheef_database::savage_mount::get_users_for_savage_mount(&savage_mount).await.expect("Savage mount does exist");
        response.get_mut(&savage_mount).expect("Vector should exist").append(&mut users_for_savage_mount);
    }

    if response.is_empty() {
        no_content!()
    } else {
        ok_json!(response)
    }
}

pub async fn get_savage_mounts_for_user(path: web::Path<UserPathInfo>) -> HttpResponse {
    if !user_exists(&path.username).await {
        return not_found!();
    }

    let data = sheef_database::savage_mount::get_savage_mounts_for_user(&path.username).await;
    if let Some(savage_mounts) = data {
        ok_json!(savage_mounts)
    } else {
        no_content!()
    }
}

pub async fn get_users_for_savage_mount(path: web::Path<SavageMountPathInfo>) -> HttpResponse {
    let data = sheef_database::savage_mount::get_users_for_savage_mount(&path.savage_mount).await;
    if let Some(savage_mounts) = data {
        ok_json!(savage_mounts)
    } else {
        not_found!()
    }
}

pub async fn get_my_savage_mounts(req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    get_savage_mounts_for_user(web::Path::<UserPathInfo>::from(UserPathInfo { username })).await
}

pub async fn activate_savage_mount_for_user(path: web::Path<SavageMountUsernamePathInfo>) -> HttpResponse {
    if !savage_mount_exists(&path.savage_mount).await || !user_exists(&path.username).await {
        return not_found!();
    }

    let data = sheef_database::savage_mount::activate_savage_mount_for_user(&path.savage_mount, &path.username).await;
    no_content_or_internal_server_error!(data)
}

pub async fn activate_savage_mount_for_me(path: web::Path<SavageMountPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    activate_savage_mount_for_user(web::Path::<SavageMountUsernamePathInfo>::from(SavageMountUsernamePathInfo { username, savage_mount: path.savage_mount.to_string() })).await
}

pub async fn deactivate_savage_mount_for_user(path: web::Path<SavageMountUsernamePathInfo>) -> HttpResponse {
    if !savage_mount_exists(&path.savage_mount).await || !user_exists(&path.username).await {
        return not_found!();
    }

    let data = sheef_database::savage_mount::deactivate_savage_mount_for_user(&path.savage_mount, &path.username).await;
    no_content_or_internal_server_error!(data)
}

pub async fn deactivate_savage_mount_for_me(path: web::Path<SavageMountPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    deactivate_savage_mount_for_user(web::Path::<SavageMountUsernamePathInfo>::from(SavageMountUsernamePathInfo { username, savage_mount: path.savage_mount.to_string() })).await
}

pub async fn delete_savage_mount(path: web::Path<SavageMountPathInfo>) -> HttpResponse {
    if !savage_mount_exists(&path.savage_mount).await {
        return not_found!();
    }

    let data = sheef_database::savage_mount::delete_savage_mount(&path.savage_mount).await;
    no_content_or_internal_server_error!(data)
}

pub async fn create_savage_mount(body: web::Json<SavageMount>) -> HttpResponse {
    let savage_mount = body.name.to_string();
    if savage_mount_exists(&body.name).await {
        return conflict!();
    }

    let data = sheef_database::savage_mount::create_savage_mount(&body.name).await;
    if let Ok(_) = data {
        created_json!(SavageMount { name: savage_mount })
    } else {
        internal_server_error!()
    }
}

pub async fn update_savage_mount(path: web::Path<SavageMountPathInfo>, body: web::Json<SavageMount>) -> HttpResponse {
    if !savage_mount_exists(&path.savage_mount).await {
        return not_found!();
    }

    let data = sheef_database::savage_mount::update_savage_mount(&path.savage_mount, &body.name).await;
    no_content_or_internal_server_error!(data)
}
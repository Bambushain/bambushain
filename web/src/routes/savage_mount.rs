use std::collections::BTreeMap;
use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;
use sheef_database::savage_mount::savage_mount_exists;
use sheef_database::user::user_exists;
use sheef_entities::Savage_Mount;
use crate::routes::user::UserPathInfo;

#[derive(Deserialize)]
pub struct SavageMountUsernamePathInfo {
    pub savage_mount: String,
    pub username: String,
}

#[derive(Deserialize)]
pub struct Savage_MountPathInfo {
    pub savage_mount: String,
}

pub async fn get_savage_mounts() -> HttpResponse {
    let data = web::block(sheef_database::savage_mount::get_savage_mounts).await;
    if let Ok(savage_mounts) = data {
        if let Ok(response) = web::block(move || {
            let mut response = BTreeMap::new();
            for savage_mount in savage_mounts {
                response.insert(savage_mount.to_string(), vec![]);
                let mut users_for_savage_mount = sheef_database::savage_mount::get_users_for_savage_mount(&savage_mount).expect("Savage mount does exist");
                response.get_mut(&savage_mount).expect("Vector should exist").append(&mut users_for_savage_mount);
            }
            response
        }).await {
            return ok_json!(response);
        }
    }

    no_content!()
}

pub async fn get_savage_mounts_for_user(path: web::Path<UserPathInfo>) -> HttpResponse {
    if !user_exists(&path.username) {
        return not_found!();
    }

    let data = web::block(move || sheef_database::savage_mount::get_savage_mounts_for_user(&path.username)).await;
    if let Ok(savage_mounts) = data {
        ok_json!(savage_mounts)
    } else {
        no_content!()
    }
}

pub async fn get_users_for_savage_mount(path: web::Path<Savage_MountPathInfo>) -> HttpResponse {
    let data = web::block(move || sheef_database::savage_mount::get_users_for_savage_mount(&path.savage_mount)).await;
    if let Ok(Some(savage_mounts)) = data {
        ok_json!(savage_mounts)
    } else {
        not_found!()
    }
}

pub async fn get_my_savage_mounts(req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    get_savage_mounts_for_user(web::Path::<UserPathInfo>::from(UserPathInfo { username })).await
}

pub async fn activate_savage_mount_for_user(path: web::Path<Savage_MountUsernamePathInfo>) -> HttpResponse {
    if !savage_mount_exists(&path.savage_mount) || !user_exists(&path.username) {
        return not_found!()
    }

    let data = web::block(move || sheef_database::savage_mount::activate_savage_mount_for_user(&path.savage_mount, &path.username)).await;
    no_content_or_internal_server_error!(data)
}

pub async fn activate_savage_mount_for_me(path: web::Path<Savage_MountPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    activate_savage_mount_for_user(web::Path::<Savage_MountUsernamePathInfo>::from(Savage_MountUsernamePathInfo { username, savage_mount: path.savage_mount.to_string() })).await
}

pub async fn deactivate_savage_mount_for_user(path: web::Path<Savage_MountUsernamePathInfo>) -> HttpResponse {
    if !savage_mount_exists(&path.savage_mount) || !user_exists(&path.username) {
        return not_found!()
    }

    let data = web::block(move || sheef_database::savage_mount::deactivate_savage_mount_for_user(&path.savage_mount, &path.username)).await;
    no_content_or_internal_server_error!(data)
}

pub async fn deactivate_savage_mount_for_me(path: web::Path<Savage_MountPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    deactivate_savage_mount_for_user(web::Path::<Savage_MountUsernamePathInfo>::from(Savage_MountUsernamePathInfo { username, savage_mount: path.savage_mount.to_string() })).await
}

pub async fn delete_savage_mount(path: web::Path<Savage_MountPathInfo>) -> HttpResponse {
    if !savage_mount_exists(&path.savage_mount) {
        return not_found!()
    }

    let data = web::block(move || sheef_database::savage_mount::delete_savage_mount(&path.savage_mount)).await;
    no_content_or_internal_server_error!(data)
}

pub async fn create_savage_mount(body: web::Json<Savage_Mount>) -> HttpResponse {
    let savage_mount = body.name.to_string();
    let data = web::block(move || sheef_database::savage_mount::create_savage_mount(&body.name)).await;
    if let Ok(Ok(_)) = data {
        created_json!(Savage_Mount { name: savage_mount })
    } else {
        internal_server_error!()
    }
}

pub async fn update_savage_mount(path: web::Path<Savage_MountPathInfo>, body: web::Json<Savage_Mount>) -> HttpResponse {
    if !savage_mount_exists(&path.savage_mount) {
        return not_found!()
    }

    let data = web::block(move || sheef_database::savage_mount::update_savage_mount(&path.savage_mount, &body.name)).await;
    no_content_or_internal_server_error!(data)
}
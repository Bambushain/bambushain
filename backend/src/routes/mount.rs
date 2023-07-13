use std::collections::BTreeMap;
use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;
use sheef_database::mount::mount_exists;
use sheef_database::user::user_exists;
use sheef_entities::Mount;
use crate::routes::user::UserPathInfo;

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
    let data = web::block(sheef_database::mount::get_mounts).await;
    if let Ok(mounts) = data {
        if let Ok(response) = web::block(move || {
            let mut response = BTreeMap::new();
            for mount in mounts {
                response.insert(mount.to_string(), vec![]);
                let mut users_for_mount = sheef_database::mount::get_users_for_mount(&mount).expect("Mount does exist");
                response.get_mut(&mount).expect("Vector should exist").append(&mut users_for_mount);
            }
            response
        }).await {
            return ok_json!(response);
        }
    }

    no_content!()
}

pub async fn get_mounts_for_user(path: web::Path<UserPathInfo>) -> HttpResponse {
    if !user_exists(&path.username) {
        return not_found!();
    }

    let data = web::block(move || sheef_database::mount::get_mounts_for_user(&path.username)).await;
    if let Ok(mounts) = data {
        ok_json!(mounts)
    } else {
        no_content!()
    }
}

pub async fn get_users_for_mount(path: web::Path<MountPathInfo>) -> HttpResponse {
    let data = web::block(move || sheef_database::mount::get_users_for_mount(&path.mount)).await;
    if let Ok(Some(mounts)) = data {
        ok_json!(mounts)
    } else {
        not_found!()
    }
}

pub async fn get_my_mounts(req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    get_mounts_for_user(web::Path::<UserPathInfo>::from(UserPathInfo { username })).await
}

pub async fn activate_mount_for_user(path: web::Path<MountUsernamePathInfo>) -> HttpResponse {
    if !mount_exists(&path.mount) || !user_exists(&path.username) {
        return not_found!()
    }

    let data = web::block(move || sheef_database::mount::activate_mount_for_user(&path.mount, &path.username)).await;
    no_content_or_internal_server_error!(data)
}

pub async fn activate_mount_for_me(path: web::Path<MountPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    activate_mount_for_user(web::Path::<MountUsernamePathInfo>::from(MountUsernamePathInfo { username, mount: path.mount.to_string() })).await
}

pub async fn deactivate_mount_for_user(path: web::Path<MountUsernamePathInfo>) -> HttpResponse {
    if !mount_exists(&path.mount) || !user_exists(&path.username) {
        return not_found!()
    }

    let data = web::block(move || sheef_database::mount::deactivate_mount_for_user(&path.mount, &path.username)).await;
    no_content_or_internal_server_error!(data)
}

pub async fn deactivate_mount_for_me(path: web::Path<MountPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    deactivate_mount_for_user(web::Path::<MountUsernamePathInfo>::from(MountUsernamePathInfo { username, mount: path.mount.to_string() })).await
}

pub async fn delete_mount(path: web::Path<MountPathInfo>) -> HttpResponse {
    if !mount_exists(&path.mount) {
        return not_found!()
    }

    let data = web::block(move || sheef_database::mount::delete_mount(&path.mount)).await;
    no_content_or_internal_server_error!(data)
}

pub async fn create_mount(body: web::Json<Mount>) -> HttpResponse {
    let mount = body.name.to_string();
    let data = web::block(move || sheef_database::mount::create_mount(&body.name)).await;
    if let Ok(Ok(_)) = data {
        created_json!(Mount { name: mount })
    } else {
        internal_server_error!()
    }
}

pub async fn update_mount(path: web::Path<MountPathInfo>, body: web::Json<Mount>) -> HttpResponse {
    if !mount_exists(&path.mount) {
        return not_found!()
    }

    let data = web::block(move || sheef_database::mount::update_mount(&path.mount, &body.name)).await;
    no_content_or_internal_server_error!(data)
}
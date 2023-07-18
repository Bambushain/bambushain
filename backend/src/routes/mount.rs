use std::collections::BTreeMap;
use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;
use sheef_database::mount::mount_exists;
use sheef_database::user::user_exists;
use sheef_entities::{Mount, sheef_exists_already_error, sheef_not_found_error, SheefError};

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
    let mounts = match sheef_database::mount::get_mounts().await {
        Ok(mounts) => mounts,
        Err(err) => return ok_or_error!(Err::<(), SheefError>(err))
    };
    let mut response = BTreeMap::new();
    for mount in mounts {
        response.insert(mount.to_string(), vec![]);
        let mut users_for_mount = sheef_database::mount::get_users_for_mount(&mount).await.expect("Mount does exist");
        response.get_mut(&mount).expect("Vector should exist").append(&mut users_for_mount);
    }

    ok_or_error!(Ok::<BTreeMap<String, Vec<String>>, SheefError>(response))
}

pub async fn activate_mount_for_user(path: web::Path<MountUsernamePathInfo>) -> HttpResponse {
    if !user_exists(&path.username).await {
        return not_found!(sheef_not_found_error!("user", "User not found"));
    }

    if !mount_exists(&path.mount).await {
        return not_found!(sheef_not_found_error!("mount", "Mount not found"));
    }

    no_content_or_error!(sheef_database::mount::activate_mount_for_user(&path.mount, &path.username).await)
}

pub async fn activate_mount_for_me(path: web::Path<MountPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    activate_mount_for_user(web::Path::<MountUsernamePathInfo>::from(MountUsernamePathInfo { username, mount: path.mount.to_string() })).await
}

pub async fn deactivate_mount_for_user(path: web::Path<MountUsernamePathInfo>) -> HttpResponse {
    if !user_exists(&path.username).await {
        return not_found!(sheef_not_found_error!("user", "User not found"));
    }

    if !mount_exists(&path.mount).await {
        return not_found!(sheef_not_found_error!("mount", "Mount not found"));
    }

    no_content_or_error!(sheef_database::mount::deactivate_mount_for_user(&path.mount, &path.username).await)
}

pub async fn deactivate_mount_for_me(path: web::Path<MountPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    deactivate_mount_for_user(web::Path::<MountUsernamePathInfo>::from(MountUsernamePathInfo { username, mount: path.mount.to_string() })).await
}

pub async fn delete_mount(path: web::Path<MountPathInfo>) -> HttpResponse {
    if !mount_exists(&path.mount).await {
        return not_found!(sheef_not_found_error!("mount", "Mount not found"));
    }

    no_content_or_error!(sheef_database::mount::delete_mount(&path.mount).await)
}

pub async fn create_mount(body: web::Json<Mount>) -> HttpResponse {
    let mount = body.name.to_string();
    if mount_exists(&body.name).await {
        return conflict!(sheef_exists_already_error!("mount", "Mount already exists"));
    }

    created_or_error!(sheef_database::mount::create_mount(&body.name).await.map(|_| Mount { name: mount }))
}

pub async fn update_mount(path: web::Path<MountPathInfo>, body: web::Json<Mount>) -> HttpResponse {
    if !mount_exists(&path.mount).await {
        return not_found!(sheef_not_found_error!("mount", "Mount not found"));
    }

    if mount_exists(&body.name).await && body.name != path.mount {
        return conflict!(sheef_exists_already_error!("mount", "Mount already exists"));
    }

    no_content_or_error!(sheef_database::mount::update_mount(&path.mount, &body.name).await)
}
use std::collections::BTreeMap;
use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;
use sheef_database::kill::kill_exists;
use sheef_database::user::user_exists;
use sheef_entities::Kill;

#[derive(Deserialize)]
pub struct KillUsernamePathInfo {
    pub kill: String,
    pub username: String,
}

#[derive(Deserialize)]
pub struct KillPathInfo {
    pub kill: String,
}

pub async fn get_kills() -> HttpResponse {
    let kills = sheef_database::kill::get_kills().await;
    let mut response = BTreeMap::new();
    for kill in kills {
        response.insert(kill.to_string(), vec![]);
        let mut users_for_kill = sheef_database::kill::get_users_for_kill(&kill).await.expect("Kill does exist");
        response.get_mut(&kill).expect("Vector should exist").append(&mut users_for_kill);
    }

    if response.is_empty() {
        no_content!()
    } else {
        ok_json!(response)
    }
}

pub async fn activate_kill_for_user(path: web::Path<KillUsernamePathInfo>) -> HttpResponse {
    if !kill_exists(&path.kill).await || !user_exists(&path.username).await {
        return not_found!();
    }

    let data = sheef_database::kill::activate_kill_for_user(&path.kill, &path.username).await;
    no_content_or_internal_server_error!(data)
}

pub async fn activate_kill_for_me(path: web::Path<KillPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    activate_kill_for_user(web::Path::<KillUsernamePathInfo>::from(KillUsernamePathInfo { username, kill: path.kill.to_string() })).await
}

pub async fn deactivate_kill_for_user(path: web::Path<KillUsernamePathInfo>) -> HttpResponse {
    if !kill_exists(&path.kill).await || !user_exists(&path.username).await {
        return not_found!();
    }

    let data = sheef_database::kill::deactivate_kill_for_user(&path.kill, &path.username).await;
    no_content_or_internal_server_error!(data)
}

pub async fn deactivate_kill_for_me(path: web::Path<KillPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    deactivate_kill_for_user(web::Path::<KillUsernamePathInfo>::from(KillUsernamePathInfo { username, kill: path.kill.to_string() })).await
}

pub async fn delete_kill(path: web::Path<KillPathInfo>) -> HttpResponse {
    if !kill_exists(&path.kill).await {
        return not_found!();
    }

    let data = sheef_database::kill::delete_kill(&path.kill).await;
    no_content_or_internal_server_error!(data)
}

pub async fn create_kill(body: web::Json<Kill>) -> HttpResponse {
    let kill = body.name.to_string();
    if kill_exists(&body.name).await {
        return conflict!();
    }

    let data = sheef_database::kill::create_kill(&body.name).await;
    if data.is_ok() {
        created_json!(Kill { name: kill })
    } else {
        internal_server_error!()
    }
}

pub async fn update_kill(path: web::Path<KillPathInfo>, body: web::Json<Kill>) -> HttpResponse {
    if !kill_exists(&path.kill).await {
        return not_found!();
    }

    if kill_exists(&body.name).await && body.name != path.kill {
        return conflict!();
    }

    let data = sheef_database::kill::update_kill(&path.kill, &body.name).await;
    no_content_or_internal_server_error!(data)
}
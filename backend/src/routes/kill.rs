use std::collections::BTreeMap;
use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;
use sheef_database::kill::kill_exists;
use sheef_database::user::user_exists;
use sheef_entities::{Kill, sheef_exists_already_error, sheef_not_found_error, SheefError};

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
    let kills = match sheef_database::kill::get_kills().await {
        Ok(kills) => kills,
        Err(err) => return ok_or_error!(Err::<(), SheefError>(err))
    };
    let mut response = BTreeMap::new();
    for kill in kills {
        response.insert(kill.to_string(), vec![]);
        let mut users_for_kill = sheef_database::kill::get_users_for_kill(&kill).await.expect("Kill does exist");
        response.get_mut(&kill).expect("Vector should exist").append(&mut users_for_kill);
    }

    ok_or_error!(Ok::<BTreeMap<String, Vec<String>>, SheefError>(response))
}

pub async fn activate_kill_for_user(path: web::Path<KillUsernamePathInfo>) -> HttpResponse {
    if !user_exists(&path.username).await {
        return not_found!(sheef_not_found_error!("user", "User not found"));
    }

    if !kill_exists(&path.kill).await {
        return not_found!(sheef_not_found_error!("kill", "Kill not found"));
    }

    no_content_or_error!(sheef_database::kill::activate_kill_for_user(&path.kill, &path.username).await)
}

pub async fn activate_kill_for_me(path: web::Path<KillPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    activate_kill_for_user(web::Path::<KillUsernamePathInfo>::from(KillUsernamePathInfo { username, kill: path.kill.to_string() })).await
}

pub async fn deactivate_kill_for_user(path: web::Path<KillUsernamePathInfo>) -> HttpResponse {
    if !user_exists(&path.username).await {
        return not_found!(sheef_not_found_error!("user", "User not found"));
    }

    if !kill_exists(&path.kill).await {
        return not_found!(sheef_not_found_error!("kill", "Kill not found"));
    }

    no_content_or_error!(sheef_database::kill::deactivate_kill_for_user(&path.kill, &path.username).await)
}

pub async fn deactivate_kill_for_me(path: web::Path<KillPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    deactivate_kill_for_user(web::Path::<KillUsernamePathInfo>::from(KillUsernamePathInfo { username, kill: path.kill.to_string() })).await
}

pub async fn delete_kill(path: web::Path<KillPathInfo>) -> HttpResponse {
    if !kill_exists(&path.kill).await {
        return not_found!(sheef_not_found_error!("kill", "Kill not found"));
    }

    no_content_or_error!(sheef_database::kill::delete_kill(&path.kill).await)
}

pub async fn create_kill(body: web::Json<Kill>) -> HttpResponse {
    let kill = body.name.to_string();
    if kill_exists(&body.name).await {
        return conflict!(sheef_exists_already_error!("kill", "Kill already exists"));
    }

    created_or_error!(sheef_database::kill::create_kill(&body.name).await.map(|_| Kill { name: kill }))
}

pub async fn update_kill(path: web::Path<KillPathInfo>, body: web::Json<Kill>) -> HttpResponse {
    if !kill_exists(&path.kill).await {
        return not_found!(sheef_not_found_error!("kill", "Kill not found"));
    }

    if kill_exists(&body.name).await && body.name != path.kill {
        return conflict!(sheef_exists_already_error!("kill", "Kill already exists"));
    }

    no_content_or_error!(sheef_database::kill::update_kill(&path.kill, &body.name).await)
}
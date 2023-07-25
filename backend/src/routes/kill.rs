use std::collections::BTreeMap;

use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;

use sheef_dbal::prelude::*;
use sheef_entities::prelude::*;

use crate::sse::NotificationState;

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
    let kills = match sheef_dbal::kill::get_kills().await {
        Ok(kills) => kills,
        Err(err) => return ok_or_error!(Err::<(), SheefError>(err))
    };
    let mut response = BTreeMap::new();
    for kill in kills {
        response.insert(kill.name.clone(), vec![]);
        let mut users_for_kill = get_users_for_kill(kill.name.clone()).await.expect("Kill does exist");
        response.get_mut(kill.name.as_str()).expect("Vector should exist").append(&mut users_for_kill);
    }

    ok_or_error!(Ok::<BTreeMap<String, Vec<String>>, SheefError>(response))
}

pub async fn activate_kill_for_user(path: web::Path<KillUsernamePathInfo>, notification_state: web::Data<NotificationState>) -> HttpResponse {
    if !user_exists(path.username.clone()).await {
        return not_found!(sheef_not_found_error!("user", "User not found"));
    }

    if !kill_exists(path.kill.clone()).await {
        return not_found!(sheef_not_found_error!("kill", "Kill not found"));
    }

    let data = sheef_dbal::kill::activate_kill_for_user(path.kill.clone(), path.username.clone()).await;
    actix_web::rt::spawn(async move {
        notification_state.kill_broadcaster.notify_change().await;
    });

    no_content_or_error!(data)
}

pub async fn activate_kill_for_me(path: web::Path<KillPathInfo>, notification_state: web::Data<NotificationState>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    activate_kill_for_user(web::Path::<KillUsernamePathInfo>::from(KillUsernamePathInfo { username, kill: path.kill.to_string() }), notification_state).await
}

pub async fn deactivate_kill_for_user(path: web::Path<KillUsernamePathInfo>, notification_state: web::Data<NotificationState>) -> HttpResponse {
    if !user_exists(path.username.clone()).await {
        return not_found!(sheef_not_found_error!("user", "User not found"));
    }

    if !kill_exists(path.kill.clone()).await {
        return not_found!(sheef_not_found_error!("kill", "Kill not found"));
    }

    let data = sheef_dbal::kill::deactivate_kill_for_user(path.kill.clone(), path.username.clone()).await;
    actix_web::rt::spawn(async move {
        notification_state.kill_broadcaster.notify_change().await;
    });

    no_content_or_error!(data)
}

pub async fn deactivate_kill_for_me(path: web::Path<KillPathInfo>, notification_state: web::Data<NotificationState>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    deactivate_kill_for_user(web::Path::<KillUsernamePathInfo>::from(KillUsernamePathInfo { username, kill: path.kill.to_string() }), notification_state).await
}

pub async fn delete_kill(path: web::Path<KillPathInfo>, notification_state: web::Data<NotificationState>) -> HttpResponse {
    if !kill_exists(path.kill.clone()).await {
        return not_found!(sheef_not_found_error!("kill", "Kill not found"));
    }

    let data = sheef_dbal::kill::delete_kill(path.kill.clone()).await;
    actix_web::rt::spawn(async move {
        notification_state.kill_broadcaster.notify_change().await;
    });

    no_content_or_error!(data)
}

pub async fn create_kill(body: web::Json<Kill>, notification_state: web::Data<NotificationState>) -> HttpResponse {
    if kill_exists(body.name.clone()).await {
        return conflict!(sheef_exists_already_error!("kill", "Kill already exists"));
    }

    let data = sheef_dbal::kill::create_kill(body.into_inner()).await.map(|kill| kill);
    actix_web::rt::spawn(async move {
        notification_state.kill_broadcaster.notify_change().await;
    });

    created_or_error!(data)
}

pub async fn update_kill(path: web::Path<KillPathInfo>, body: web::Json<Kill>, notification_state: web::Data<NotificationState>) -> HttpResponse {
    if !kill_exists(path.kill.clone()).await {
        return not_found!(sheef_not_found_error!("kill", "Kill not found"));
    }

    if kill_exists(body.name.clone()).await && body.name != path.kill {
        return conflict!(sheef_exists_already_error!("kill", "Kill already exists"));
    }

    let data = sheef_dbal::kill::update_kill(path.kill.clone(), body.name.clone()).await;
    actix_web::rt::spawn(async move {
        notification_state.kill_broadcaster.notify_change().await;
    });

    no_content_or_error!(data)
}

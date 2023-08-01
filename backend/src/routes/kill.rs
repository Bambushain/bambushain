use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;

use pandaparty_dbal::prelude::*;
use pandaparty_entities::prelude::*;

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
    ok_or_error!(pandaparty_dbal::kill::get_kills().await)
}

pub async fn activate_kill_for_user(path: web::Path<KillUsernamePathInfo>, notification_state: web::Data<NotificationState>) -> HttpResponse {
    if !user_exists(path.username.clone()).await {
        return not_found!(pandaparty_not_found_error!("user", "User not found"));
    }

    if !kill_exists(path.kill.clone()).await {
        return not_found!(pandaparty_not_found_error!("kill", "Kill not found"));
    }

    let data = pandaparty_dbal::kill::activate_kill_for_user(path.kill.clone(), path.username.clone()).await;
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
        return not_found!(pandaparty_not_found_error!("user", "User not found"));
    }

    if !kill_exists(path.kill.clone()).await {
        return not_found!(pandaparty_not_found_error!("kill", "Kill not found"));
    }

    let data = pandaparty_dbal::kill::deactivate_kill_for_user(path.kill.clone(), path.username.clone()).await;
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
        return not_found!(pandaparty_not_found_error!("kill", "Kill not found"));
    }

    let data = pandaparty_dbal::kill::delete_kill(path.kill.clone()).await;
    actix_web::rt::spawn(async move {
        notification_state.kill_broadcaster.notify_change().await;
    });

    no_content_or_error!(data)
}

pub async fn create_kill(body: web::Json<Kill>, notification_state: web::Data<NotificationState>) -> HttpResponse {
    if kill_exists(body.name.clone()).await {
        return conflict!(pandaparty_exists_already_error!("kill", "Kill already exists"));
    }

    let data = pandaparty_dbal::kill::create_kill(body.into_inner()).await.map(|kill| kill);
    actix_web::rt::spawn(async move {
        notification_state.kill_broadcaster.notify_change().await;
    });

    created_or_error!(data)
}

pub async fn update_kill(path: web::Path<KillPathInfo>, body: web::Json<Kill>, notification_state: web::Data<NotificationState>) -> HttpResponse {
    if !kill_exists(path.kill.clone()).await {
        return not_found!(pandaparty_not_found_error!("kill", "Kill not found"));
    }

    if kill_exists(body.name.clone()).await && body.name != path.kill {
        return conflict!(pandaparty_exists_already_error!("kill", "Kill already exists"));
    }

    let data = pandaparty_dbal::kill::update_kill(path.kill.clone(), body.name.clone()).await;
    actix_web::rt::spawn(async move {
        notification_state.kill_broadcaster.notify_change().await;
    });

    no_content_or_error!(data)
}

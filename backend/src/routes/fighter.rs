use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;
use sheef_database::fighter::fighter_exists;
use sheef_entities::Fighter;

#[derive(Deserialize)]
pub struct FighterPathInfo {
    pub job: String,
}

pub async fn get_fighters(req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    let data = sheef_database::fighter::get_fighters(&username).await;
    ok_or_not_found!(data)
}

pub async fn get_fighter(path: web::Path<FighterPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    let data = sheef_database::fighter::get_fighter(&username, &path.job).await;
    ok_or_not_found!(data)
}

pub async fn create_fighter(body: web::Json<Fighter>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    if fighter_exists(&username, &body.job).await {
        return conflict!();
    }

    let data = sheef_database::fighter::create_fighter(&username, &body.job, &body.level, &body.gear_score).await;
    if let Some(fighter) = data {
        created_json!(fighter)
    } else {
        internal_server_error!()
    }
}

pub async fn update_fighter(body: web::Json<Fighter>, path: web::Path<FighterPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    if !fighter_exists(&username, &path.job).await {
        return not_found!();
    }

    if fighter_exists(&username, &body.job).await && body.job != path.job {
        return conflict!();
    }

    let data = sheef_database::fighter::update_fighter(&username, &path.job, &body.level, &body.gear_score, &body.job).await;
    no_content_or_internal_server_error!(data)
}

pub async fn delete_fighter(path: web::Path<FighterPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    if !fighter_exists(&username, &path.job).await {
        return not_found!();
    }

    let data = sheef_database::fighter::delete_fighter(&username, &path.job).await;
    no_content_or_internal_server_error!(data)
}

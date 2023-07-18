use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;
use sheef_database::fighter::fighter_exists;
use sheef_entities::{Fighter, sheef_exists_already_error, sheef_not_found_error};

#[derive(Deserialize)]
pub struct FighterPathInfo {
    pub job: String,
}

pub async fn get_fighters(req: HttpRequest) -> HttpResponse {
    let username = username!(req);

    ok_or_error!(sheef_database::fighter::get_fighters(&username).await)
}

pub async fn get_fighter(path: web::Path<FighterPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);

    ok_or_error!(sheef_database::fighter::get_fighter(&username, &path.job).await)
}

pub async fn create_fighter(body: web::Json<Fighter>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    if fighter_exists(&username, &body.job).await {
        return conflict!(sheef_exists_already_error!("fighter", "The fighter already exists"));
    }

    created_or_error!(sheef_database::fighter::create_fighter(&username, &body.job, &body.level, &body.gear_score).await)
}

pub async fn update_fighter(body: web::Json<Fighter>, path: web::Path<FighterPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    if !fighter_exists(&username, &path.job).await {
        return not_found!(sheef_not_found_error!("fighter", "The fighter was not found"));
    }

    if fighter_exists(&username, &body.job).await && body.job != path.job {
        return conflict!(sheef_exists_already_error!("fighter", "The fighter already exists"));
    }

    created_or_error!(sheef_database::fighter::update_fighter(&username, &path.job, &body.level, &body.gear_score, &body.job).await)
}

pub async fn delete_fighter(path: web::Path<FighterPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    if !fighter_exists(&username, &path.job).await {
        return not_found!(sheef_not_found_error!("fighter", "The fighter was not found"));
    }

    no_content_or_error!(sheef_database::fighter::delete_fighter(&username, &path.job).await)
}

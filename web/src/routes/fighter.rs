use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use actix_web::http::StatusCode;
use serde::Deserialize;
use sheef_entities::Fighter;
use sheef_entities::fighter::UpdateFighter;
use crate::middleware::authenticate_user::AuthenticationState;

#[derive(Deserialize)]
pub struct FighterPathInfo {
    pub job: String,
}

pub async fn get_fighters(req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    let data = web::block(move || sheef_database::fighter::get_fighters(&username)).await;
    if let Ok(Some(fighters)) = data {
        HttpResponse::Ok().json(web::Json(fighters))
    } else {
        HttpResponse::new(StatusCode::NOT_FOUND)
    }
}

pub async fn get_fighter(info: web::Path<FighterPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    let data = web::block(move || sheef_database::fighter::get_fighter(&username, &info.job)).await;
    if let Ok(Some(fighter)) = data {
        HttpResponse::Ok().json(web::Json(fighter))
    } else {
        HttpResponse::new(StatusCode::NOT_FOUND)
    }
}

pub async fn create_fighter(body: web::Json<Fighter>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    let data = web::block(move || sheef_database::fighter::create_fighter(&username, &body.job, &body.level, &body.gear_score)).await;
    if let Ok(Some(fighter)) = data {
        HttpResponse::Created().json(web::Json(fighter))
    } else {
        HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn update_fighter(body: web::Json<UpdateFighter>, info: web::Path<FighterPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    let data = web::block(move || sheef_database::fighter::update_fighter(&username, &info.job, &body.level, &body.gear_score)).await;
    if let Ok(Ok(_)) = data {
        HttpResponse::new(StatusCode::NO_CONTENT)
    } else {
        HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn delete_fighter(info: web::Path<FighterPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    let data = web::block(move || sheef_database::fighter::delete_fighter(&username, &info.job)).await;
    if let Ok(Ok(_)) = data {
        HttpResponse::new(StatusCode::NO_CONTENT)
    } else {
        HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

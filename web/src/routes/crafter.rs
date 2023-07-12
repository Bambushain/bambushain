use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::http::StatusCode;
use serde::Deserialize;
use sheef_entities::Crafter;
use sheef_entities::crafter::UpdateCrafter;

#[derive(Deserialize)]
pub struct CrafterPathInfo {
    pub job: String,
}

pub async fn get_crafters(req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    let data = web::block(move || sheef_database::crafter::get_crafters(&username)).await;
    if let Ok(Some(crafters)) = data {
        HttpResponse::Ok().json(web::Json(crafters))
    } else {
        HttpResponse::new(StatusCode::NOT_FOUND)
    }
}

pub async fn get_crafter(info: web::Path<CrafterPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    let data = web::block(move || sheef_database::crafter::get_crafter(&username, &info.job)).await;
    if let Ok(Some(crafter)) = data {
        HttpResponse::Ok().json(web::Json(crafter))
    } else {
        HttpResponse::new(StatusCode::NOT_FOUND)
    }
}

pub async fn create_crafter(body: web::Json<Crafter>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    let data = web::block(move || sheef_database::crafter::create_crafter(&username, &body.job, &body.level)).await;
    if let Ok(Some(crafter)) = data {
        HttpResponse::Created().json(web::Json(crafter))
    } else {
        HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn update_crafter(body: web::Json<UpdateCrafter>, info: web::Path<CrafterPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    let data = web::block(move || sheef_database::crafter::update_crafter(&username, &info.job, &body.level)).await;
    if let Ok(Ok(_)) = data {
        HttpResponse::new(StatusCode::NO_CONTENT)
    } else {
        HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn delete_crafter(info: web::Path<CrafterPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    let data = web::block(move || sheef_database::crafter::delete_crafter(&username, &info.job)).await;
    if let Ok(Ok(_)) = data {
        HttpResponse::new(StatusCode::NO_CONTENT)
    } else {
        HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

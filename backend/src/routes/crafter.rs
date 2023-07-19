use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;

use sheef_database::crafter::crafter_exists;
use sheef_entities::{Crafter, sheef_exists_already_error, sheef_not_found_error};

#[derive(Deserialize)]
pub struct CrafterPathInfo {
    pub job: String,
}

pub async fn get_crafters(req: HttpRequest) -> HttpResponse {
    let username = username!(req);

    ok_or_error!(sheef_database::crafter::get_crafters(&username).await)
}

pub async fn get_crafter(info: web::Path<CrafterPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);

    ok_or_error!(sheef_database::crafter::get_crafter(&username, &info.job).await)
}

pub async fn create_crafter(body: web::Json<Crafter>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    if crafter_exists(&username, &body.job).await {
        return conflict!(sheef_exists_already_error!("crafter", "The crafter already exists"));
    }

    created_or_error!(sheef_database::crafter::create_crafter(&username, &body.job, &body.level).await)
}

pub async fn update_crafter(body: web::Json<Crafter>, path: web::Path<CrafterPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    if !crafter_exists(&username, &path.job).await {
        return not_found!(sheef_not_found_error!("crafter", "The crafter was not found"));
    }

    if crafter_exists(&username, &body.job).await && body.job != path.job {
        return conflict!(sheef_exists_already_error!("crafter", "The crafter already exists"));
    }

    no_content_or_error!(sheef_database::crafter::update_crafter(&username, &path.job, &body.level, &body.job).await)
}

pub async fn delete_crafter(path: web::Path<CrafterPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    if !crafter_exists(&username, &path.job).await {
        return not_found!(sheef_not_found_error!("crafter", "The crafter was not found"));
    }

    no_content_or_error!(sheef_database::crafter::delete_crafter(&username, &path.job).await)
}

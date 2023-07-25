use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;

use sheef_entities::prelude::*;

#[derive(Deserialize)]
pub struct CrafterPathInfo {
    pub job: String,
}

pub async fn get_crafters(req: HttpRequest) -> HttpResponse {
    let username = username!(req);

    ok_or_error!(sheef_dbal::crafter::get_crafters(username).await)
}

pub async fn get_crafter(info: web::Path<CrafterPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);

    ok_or_error!(sheef_dbal::crafter::get_crafter(username, info.job.clone()).await)
}

pub async fn create_crafter(body: web::Json<Crafter>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    if sheef_dbal::crafter::crafter_exists(username.clone(), body.job.clone()).await {
        return conflict!(sheef_exists_already_error!("crafter", "The crafter already exists"));
    }

    created_or_error!(sheef_dbal::crafter::create_crafter(username.clone(), body.into_inner()).await)
}

pub async fn update_crafter(body: web::Json<Crafter>, path: web::Path<CrafterPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    let crafter_exists = sheef_dbal::crafter::crafter_exists(username.clone(), path.job.clone()).await;
    if !crafter_exists {
        return not_found!(sheef_not_found_error!("crafter", "The crafter was not found"));
    }

    if crafter_exists && body.job != path.job {
        return conflict!(sheef_exists_already_error!("crafter", "The crafter already exists"));
    }

    no_content_or_error!(sheef_dbal::crafter::update_crafter(username.clone(), path.job.clone(), body.into_inner()).await)
}

pub async fn delete_crafter(path: web::Path<CrafterPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    if !sheef_dbal::crafter::crafter_exists(username.clone(), path.job.clone()).await {
        return not_found!(sheef_not_found_error!("crafter", "The crafter was not found"));
    }

    no_content_or_error!(sheef_dbal::crafter::delete_crafter(username, path.job.clone()).await)
}

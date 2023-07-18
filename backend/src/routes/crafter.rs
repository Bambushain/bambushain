use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;
use sheef_database::crafter::crafter_exists;
use sheef_entities::Crafter;

#[derive(Deserialize)]
pub struct CrafterPathInfo {
    pub job: String,
}

pub async fn get_crafters(req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    let data = sheef_database::crafter::get_crafters(&username).await;
    ok_or_not_found!(data)
}

pub async fn get_crafter(info: web::Path<CrafterPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    let data = sheef_database::crafter::get_crafter(&username, &info.job).await;
    ok_or_not_found!(data)
}

pub async fn create_crafter(body: web::Json<Crafter>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    if crafter_exists(&username, &body.job).await {
        return conflict!();
    }

    let data = sheef_database::crafter::create_crafter(&username, &body.job, &body.level).await;
    if let Some(crafter) = data {
        created_json!(crafter)
    } else {
        internal_server_error!()
    }
}

pub async fn update_crafter(body: web::Json<Crafter>, path: web::Path<CrafterPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    if !crafter_exists(&username, &path.job).await {
        return not_found!();
    }

    if crafter_exists(&username, &body.job).await && body.job != path.job {
        return conflict!();
    }

    let data = sheef_database::crafter::update_crafter(&username, &path.job, &body.level, &body.job).await;
    no_content_or_internal_server_error!(data)
}

pub async fn delete_crafter(path: web::Path<CrafterPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    if !crafter_exists(&username, &path.job).await {
        return not_found!();
    }

    let data = sheef_database::crafter::delete_crafter(&username, &path.job).await;
    no_content_or_internal_server_error!(data)
}

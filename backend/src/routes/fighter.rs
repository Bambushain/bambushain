use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;

use pandaparty_dbal::prelude::*;
use pandaparty_entities::prelude::*;

#[derive(Deserialize)]
pub struct FighterPathInfo {
    pub job: String,
}

pub async fn get_fighters(req: HttpRequest) -> HttpResponse {
    let username = username!(req);

    ok_or_error!(pandaparty_dbal::fighter::get_fighters(username.clone()).await)
}

pub async fn get_fighter(path: web::Path<FighterPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);

    ok_or_error!(pandaparty_dbal::fighter::get_fighter(username.clone(), path.job.clone()).await)
}

pub async fn create_fighter(body: web::Json<Fighter>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    if fighter_exists(username.clone(), body.job.clone()).await {
        return conflict!(pandaparty_exists_already_error!("fighter", "The fighter already exists"));
    }

    created_or_error!(pandaparty_dbal::fighter::create_fighter(username.clone(), body.into_inner()).await)
}

pub async fn update_fighter(body: web::Json<Fighter>, path: web::Path<FighterPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    let fighter_exists = fighter_exists(username.clone(), path.job.clone()).await;
    if !fighter_exists {
        return not_found!(pandaparty_not_found_error!("fighter", "The fighter was not found"));
    }

    if fighter_exists && body.job != path.job {
        return conflict!(pandaparty_exists_already_error!("fighter", "The fighter already exists"));
    }

    created_or_error!(pandaparty_dbal::fighter::update_fighter(username.clone(), path.job.clone(), body.into_inner()).await)
}

pub async fn delete_fighter(path: web::Path<FighterPathInfo>, req: HttpRequest) -> HttpResponse {
    let username = username!(req);
    if !fighter_exists(username.clone(), path.job.clone()).await {
        return not_found!(pandaparty_not_found_error!("fighter", "The fighter was not found"));
    }

    no_content_or_error!(pandaparty_dbal::fighter::delete_fighter(username.clone(), path.job.clone()).await)
}

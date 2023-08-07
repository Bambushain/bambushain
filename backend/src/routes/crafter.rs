use actix_web::{HttpResponse, web};
use serde::Deserialize;

use pandaparty_entities::prelude::*;

use crate::DbConnection;
use crate::middleware::authenticate_user::Authentication;

#[derive(Deserialize)]
pub struct CrafterPathInfo {
    pub id: i32,
}

pub async fn get_crafters(authentication: Authentication, db: DbConnection) -> HttpResponse {
    ok_or_error!(pandaparty_dbal::crafter::get_crafters(authentication.user.id, &db).await)
}

pub async fn get_crafter(info: web::Path<CrafterPathInfo>, db: DbConnection) -> HttpResponse {
    ok_or_error!(pandaparty_dbal::crafter::get_crafter(info.id, &db).await)
}

pub async fn create_crafter(body: web::Json<Crafter>, authentication: Authentication, db: DbConnection) -> HttpResponse {
    if pandaparty_dbal::crafter::crafter_exists_by_job(authentication.user.id, body.job, &db).await {
        return conflict!(pandaparty_exists_already_error!("crafter", "The crafter already exists"));
    }

    created_or_error!(pandaparty_dbal::crafter::create_crafter(authentication.user.id, body.into_inner(), &db).await)
}

pub async fn update_crafter(body: web::Json<Crafter>, path: web::Path<CrafterPathInfo>, authentication: Authentication, db: DbConnection) -> HttpResponse {
    let crafter = match pandaparty_dbal::crafter::get_crafter(path.id, &db).await {
        Ok(crafter) => crafter,
        Err(_) => return not_found!(pandaparty_not_found_error!("crafter", "The crafter was not found"))
    };

    if body.job != crafter.job && pandaparty_dbal::crafter::crafter_exists_by_job(authentication.user.id, body.job, &db).await{
        return conflict!(pandaparty_exists_already_error!("crafter", "The crafter already exists"));
    }

    no_content_or_error!(pandaparty_dbal::crafter::update_crafter(path.id, body.into_inner(), &db).await)
}

pub async fn delete_crafter(path: web::Path<CrafterPathInfo>, db: DbConnection) -> HttpResponse {
    if !pandaparty_dbal::crafter::crafter_exists(path.id, &db).await {
        return not_found!(pandaparty_not_found_error!("crafter", "The crafter was not found"));
    }

    no_content_or_error!(pandaparty_dbal::crafter::delete_crafter(path.id, &db).await)
}

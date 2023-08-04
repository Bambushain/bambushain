use actix_web::{HttpResponse, web};
use serde::Deserialize;

use pandaparty_entities::prelude::*;

use crate::DbConnection;
use crate::middleware::authenticate_user::Authentication;

#[derive(Deserialize)]
pub struct CrafterPathInfo {
    pub job: String,
}

pub async fn get_crafters(authentication_state: Authentication, db: DbConnection) -> HttpResponse {
    ok_or_error!(pandaparty_dbal::crafter::get_crafters(authentication_state.user.username.clone(), &db).await)
}

pub async fn get_crafter(info: web::Path<CrafterPathInfo>, authentication_state: Authentication, db: DbConnection) -> HttpResponse {
    ok_or_error!(pandaparty_dbal::crafter::get_crafter(authentication_state.user.username.clone(), info.job.clone(), &db).await)
}

pub async fn create_crafter(body: web::Json<Crafter>, authentication_state: Authentication, db: DbConnection) -> HttpResponse {
    if pandaparty_dbal::crafter::crafter_exists(authentication_state.user.username.clone(), body.job.clone(), &db).await {
        return conflict!(pandaparty_exists_already_error!("crafter", "The crafter already exists"));
    }

    created_or_error!(pandaparty_dbal::crafter::create_crafter(authentication_state.user.username.clone(), body.into_inner(), &db).await)
}

pub async fn update_crafter(body: web::Json<Crafter>, path: web::Path<CrafterPathInfo>, authentication_state: Authentication, db: DbConnection) -> HttpResponse {
    let crafter_exists = pandaparty_dbal::crafter::crafter_exists(authentication_state.user.username.clone(), path.job.clone(), &db).await;
    if !crafter_exists {
        return not_found!(pandaparty_not_found_error!("crafter", "The crafter was not found"));
    }

    if crafter_exists && body.job != path.job {
        return conflict!(pandaparty_exists_already_error!("crafter", "The crafter already exists"));
    }

    no_content_or_error!(pandaparty_dbal::crafter::update_crafter(authentication_state.user.username.clone(), path.job.clone(), body.into_inner(), &db).await)
}

pub async fn delete_crafter(path: web::Path<CrafterPathInfo>, authentication_state: Authentication, db: DbConnection) -> HttpResponse {
    if !pandaparty_dbal::crafter::crafter_exists(authentication_state.user.username.clone(), path.job.clone(), &db).await {
        return not_found!(pandaparty_not_found_error!("crafter", "The crafter was not found"));
    }

    no_content_or_error!(pandaparty_dbal::crafter::delete_crafter(authentication_state.user.username.clone(), path.job.clone(), &db).await)
}

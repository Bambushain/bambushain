use actix_web::{HttpResponse, web};
use serde::Deserialize;

use pandaparty_entities::prelude::*;

use crate::DbConnection;
use crate::middleware::authenticate_user::Authentication;

#[derive(Deserialize)]
pub struct FighterPathInfo {
    pub id: i32,
}

pub async fn get_fighters(authentication: Authentication, db: DbConnection) -> HttpResponse {
    ok_or_error!(pandaparty_dbal::fighter::get_fighters(authentication.user.id, &db).await)
}

pub async fn get_fighter(path: web::Path<FighterPathInfo>, db: DbConnection) -> HttpResponse {
    ok_or_error!(pandaparty_dbal::fighter::get_fighter(path.id, &db).await)
}

pub async fn create_fighter(body: web::Json<Fighter>, authentication: Authentication, db: DbConnection) -> HttpResponse {
    if pandaparty_dbal::fighter::fighter_exists_by_job(authentication.user.id, body.job, &db).await {
        return conflict!(pandaparty_exists_already_error!("fighter", "The fighter already exists"));
    }

    created_or_error!(pandaparty_dbal::fighter::create_fighter(authentication.user.id, body.into_inner(), &db).await)
}

pub async fn update_fighter(body: web::Json<Fighter>, path: web::Path<FighterPathInfo>, authentication: Authentication, db: DbConnection) -> HttpResponse {
    let fighter = match pandaparty_dbal::fighter::get_fighter(path.id, &db).await {
        Ok(fighter) => fighter,
        Err(_) => return not_found!(pandaparty_not_found_error!("fighter", "The fighter was not found"))
    };

    if body.job != fighter.job && pandaparty_dbal::fighter::fighter_exists_by_job(authentication.user.id, body.job, &db).await {
        return conflict!(pandaparty_exists_already_error!("fighter", "The fighter already exists"));
    }

    no_content_or_error!(pandaparty_dbal::fighter::update_fighter(path.id, body.into_inner(), &db).await)
}

pub async fn delete_fighter(path: web::Path<FighterPathInfo>, db: DbConnection) -> HttpResponse {
    if !pandaparty_dbal::fighter::fighter_exists(path.id, &db).await {
        return not_found!(pandaparty_not_found_error!("fighter", "The fighter was not found"));
    }

    no_content_or_error!(pandaparty_dbal::fighter::delete_fighter(path.id, &db).await)
}

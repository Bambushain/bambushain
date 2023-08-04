use actix_web::{HttpResponse, web};
use serde::Deserialize;

use pandaparty_dbal::prelude::*;
use pandaparty_entities::prelude::*;

use crate::DbConnection;
use crate::middleware::authenticate_user::Authentication;

#[derive(Deserialize)]
pub struct FighterPathInfo {
    pub job: String,
}

pub async fn get_fighters(authentication_state: Authentication, db: DbConnection) -> HttpResponse {
    ok_or_error!(pandaparty_dbal::fighter::get_fighters(authentication_state.user.username.clone(), &db).await)
}

pub async fn get_fighter(path: web::Path<FighterPathInfo>, authentication_state: Authentication, db: DbConnection) -> HttpResponse {
    ok_or_error!(pandaparty_dbal::fighter::get_fighter(authentication_state.user.username.clone(), path.job.clone(), &db).await)
}

pub async fn create_fighter(body: web::Json<Fighter>, authentication_state: Authentication, db: DbConnection) -> HttpResponse {
    if fighter_exists(authentication_state.user.username.clone(), body.job.clone(), &db).await {
        return conflict!(pandaparty_exists_already_error!("fighter", "The fighter already exists"));
    }

    created_or_error!(pandaparty_dbal::fighter::create_fighter(authentication_state.user.username.clone(), body.into_inner(), &db).await)
}

pub async fn update_fighter(body: web::Json<Fighter>, path: web::Path<FighterPathInfo>, authentication_state: Authentication, db: DbConnection) -> HttpResponse {
    let fighter_exists = fighter_exists(authentication_state.user.username.clone(), path.job.clone(), &db).await;
    if !fighter_exists {
        return not_found!(pandaparty_not_found_error!("fighter", "The fighter was not found"));
    }

    if fighter_exists && body.job != path.job {
        return conflict!(pandaparty_exists_already_error!("fighter", "The fighter already exists"));
    }

    created_or_error!(pandaparty_dbal::fighter::update_fighter(authentication_state.user.username.clone(), path.job.clone(), body.into_inner(), &db).await)
}

pub async fn delete_fighter(path: web::Path<FighterPathInfo>, authentication_state: Authentication, db: DbConnection) -> HttpResponse {
    if !fighter_exists(authentication_state.user.username.clone(), path.job.clone(), &db).await {
        return not_found!(pandaparty_not_found_error!("fighter", "The fighter was not found"));
    }

    no_content_or_error!(pandaparty_dbal::fighter::delete_fighter(authentication_state.user.username.clone(), path.job.clone(), &db).await)
}

use bamboo_services::prelude::*;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

macro_rules! no_content {
    () => {{
        actix_web::HttpResponse::new(actix_web::http::StatusCode::NO_CONTENT)
    }};
}

macro_rules! bad_request {
    () => {{
        actix_web::HttpResponse::new(actix_web::http::StatusCode::BAD_REQUEST)
    }};
    ($err:expr) => {{
        actix_web::HttpResponse::BadRequest().json($err)
    }};
}

macro_rules! unauthorized {
    () => {{
        actix_web::HttpResponse::new(actix_web::http::StatusCode::UNAUTHORIZED)
    }};
    ($err:expr) => {{
        actix_web::HttpResponse::Unauthorized().json($err)
    }};
}

macro_rules! forbidden {
    () => {{
        actix_web::HttpResponse::new(actix_web::http::StatusCode::FORBIDDEN)
    }};
    ($err:expr) => {{
        actix_web::HttpResponse::Forbidden().json($err)
    }};
}

macro_rules! not_found {
    () => {{
        actix_web::HttpResponse::new(actix_web::http::StatusCode::NOT_FOUND)
    }};
    ($err:expr) => {{
        actix_web::HttpResponse::NotFound().json($err)
    }};
}

macro_rules! conflict {
    () => {{
        actix_web::HttpResponse::new(actix_web::http::StatusCode::CONFLICT)
    }};
    ($err:expr) => {{
        actix_web::HttpResponse::Conflict().json($err)
    }};
}

macro_rules! internal_server_error {
    () => {{
        actix_web::HttpResponse::new(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
    }};
    ($err:expr) => {{
        actix_web::HttpResponse::InternalServerError().json($err)
    }};
}

macro_rules! error_response {
    ($err:expr) => {
        match $err.error_type {
            bamboo_entities::prelude::BambooErrorCode::NotFoundError => not_found!($err),
            bamboo_entities::prelude::BambooErrorCode::ExistsAlreadyError => {
                conflict!($err)
            }
            bamboo_entities::prelude::BambooErrorCode::InsufficientRightsError => {
                forbidden!($err)
            }
            bamboo_entities::prelude::BambooErrorCode::UnauthorizedError => {
                unauthorized!($err)
            }
            bamboo_entities::prelude::BambooErrorCode::InvalidDataError
            | bamboo_entities::prelude::BambooErrorCode::ValidationError => {
                bad_request!($err)
            }
            bamboo_entities::prelude::BambooErrorCode::DbError
            | bamboo_entities::prelude::BambooErrorCode::IoError
            | bamboo_entities::prelude::BambooErrorCode::SerializationError
            | bamboo_entities::prelude::BambooErrorCode::UnknownError => {
                internal_server_error!($err)
            }
        }
    };
}

macro_rules! no_content_or_error {
    ($data:expr) => {{
        match $data {
            Ok(_) => no_content!(),
            Err(err) => error_response!(err),
        }
    }};
}

macro_rules! ok_or_error {
    ($data:expr) => {{
        match $data {
            Ok(data) => ok_json!(data),
            Err(err) => error_response!(err),
        }
    }};
}

macro_rules! created_or_error {
    ($data:expr) => {{
        match $data {
            Ok(data) => created_json!(data),
            Err(err) => error_response!(err),
        }
    }};
}

macro_rules! ok_json {
    ($data:expr) => {{
        actix_web::HttpResponse::Ok().json($data)
    }};
}

macro_rules! created_json {
    ($data:expr) => {{
        actix_web::HttpResponse::Created().json($data)
    }};
}

pub type DbConnection = actix_web::web::Data<DatabaseConnection>;

#[derive(Default, Clone)]
pub struct ServicesState {
    pub environment_service: Arc<EnvironmentService>,
}

pub type Services = actix_web::web::Data<ServicesState>;

pub mod broadcaster;
pub mod middleware;
pub mod routes;
pub mod sse;

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
            bamboo_error::BambooErrorCode::NotFoundError => not_found!($err),
            bamboo_error::BambooErrorCode::ExistsAlreadyError => {
                conflict!($err)
            }
            bamboo_error::BambooErrorCode::InsufficientRightsError => {
                forbidden!($err)
            }
            bamboo_error::BambooErrorCode::UnauthorizedError => {
                unauthorized!($err)
            }
            bamboo_error::BambooErrorCode::InvalidDataError
            | bamboo_error::BambooErrorCode::ValidationError => {
                bad_request!($err)
            }
            bamboo_error::BambooErrorCode::DbError
            | bamboo_error::BambooErrorCode::IoError
            | bamboo_error::BambooErrorCode::SerializationError
            | bamboo_error::BambooErrorCode::CryptoError
            | bamboo_error::BambooErrorCode::UnknownError => {
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

macro_rules! missing_fields {
    ($entity:expr) => {{
        actix_web::HttpResponse::BadRequest().json(bamboo_error::BambooError {
            entity_type: $entity.to_string(),
            error_type: bamboo_error::BambooErrorCode::InvalidDataError,
            message: "You are missing some fields".to_string(),
        })
    }};
}

macro_rules! check_missing_fields {
    ($body:expr, $entity:expr) => {{
        if let Some(body) = $body {
            body
        } else {
            return missing_fields!($entity);
        }
    }};
}

macro_rules! invalid_path {
    ($entity:expr) => {{
        actix_web::HttpResponse::BadRequest().json(bamboo_error::BambooError {
            entity_type: $entity.to_string(),
            error_type: bamboo_error::BambooErrorCode::InvalidDataError,
            message: "You passed invalid path data".to_string(),
        })
    }};
}

macro_rules! check_invalid_path {
    ($body:expr, $entity:expr) => {{
        if let Some(body) = $body {
            body
        } else {
            return invalid_path!($entity);
        }
    }};
}

macro_rules! invalid_query {
    ($entity:expr) => {{
        actix_web::HttpResponse::BadRequest().json(bamboo_error::BambooError {
            entity_type: $entity.to_string(),
            error_type: bamboo_error::BambooErrorCode::InvalidDataError,
            message: "You passed invalid query data".to_string(),
        })
    }};
}

macro_rules! check_invalid_query {
    ($body:expr, $entity:expr) => {{
        if let Some(body) = $body {
            body
        } else {
            return invalid_query!($entity);
        }
    }};
}

mod broadcaster;
mod middleware;
mod routes;
mod sse;

pub mod prelude {
    pub use crate::broadcaster::event::EventBroadcaster;
    pub use crate::routes::configure_routes;
    pub use crate::sse::{Notification, NotificationState};
}

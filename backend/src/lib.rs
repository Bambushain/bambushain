use sea_orm::DatabaseConnection;

macro_rules! no_content {
    () => {
        {
            actix_web::HttpResponse::new(actix_web::http::StatusCode::NO_CONTENT)
        }
    };
}

macro_rules! bad_request {
    () => {
        {
            actix_web::HttpResponse::new(actix_web::http::StatusCode::BAD_REQUEST)
        }
    };
    ($err:expr) => {
        {
            actix_web::HttpResponse::BadRequest().json($err)
        }
    };
}

macro_rules! unauthorized {
    () => {
        {
            actix_web::HttpResponse::new(actix_web::http::StatusCode::UNAUTHORIZED)
        }
    };
    ($err:expr) => {
        {
            actix_web::HttpResponse::Unauthorized().json($err)
        }
    };
}

macro_rules! forbidden {
    () => {
        {
            actix_web::HttpResponse::new(actix_web::http::StatusCode::FORBIDDEN)
        }
    };
    ($err:expr) => {
        {
            actix_web::HttpResponse::Forbidden().json($err)
        }
    };
}

macro_rules! not_found {
    () => {
        {
            actix_web::HttpResponse::new(actix_web::http::StatusCode::NOT_FOUND)
        }
    };
    ($err:expr) => {
        {
            actix_web::HttpResponse::NotFound().json($err)
        }
    };
}

macro_rules! conflict {
    () => {
        {
            actix_web::HttpResponse::new(actix_web::http::StatusCode::CONFLICT)
        }
    };
    ($err:expr) => {
        {
            actix_web::HttpResponse::Conflict().json($err)
        }
    };
}

macro_rules! internal_server_error {
    () => {
        {
            actix_web::HttpResponse::new(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    };
    ($err:expr) => {
        {
            actix_web::HttpResponse::InternalServerError().json($err)
        }
    };
}

macro_rules! no_content_or_error {
    ($data:expr) => {
        {
            match $data {
                Ok(_) => no_content!(),
                Err(err) => match err.error_type {
                    pandaparty_entities::prelude::PandaPartyErrorCode::NotFoundError => not_found!(err),
                    pandaparty_entities::prelude::PandaPartyErrorCode::ExistsAlreadyError => conflict!(err),
                    pandaparty_entities::prelude::PandaPartyErrorCode::InsufficientRightsError => forbidden!(err),
                    pandaparty_entities::prelude::PandaPartyErrorCode::UnauthorizedError => unauthorized!(err),
                    pandaparty_entities::prelude::PandaPartyErrorCode::InvalidDataError | pandaparty_entities::prelude::PandaPartyErrorCode::ValidationError => bad_request!(err),
                    pandaparty_entities::prelude::PandaPartyErrorCode::DbError | pandaparty_entities::prelude::PandaPartyErrorCode::IoError | pandaparty_entities::prelude::PandaPartyErrorCode::SerializationError | pandaparty_entities::prelude::PandaPartyErrorCode::UnknownError => internal_server_error!(err),
                }
            }
        }
    };
}

macro_rules! ok_or_error {
    ($data:expr) => {
        {
            match $data {
                Ok(data) => ok_json!(data),
                Err(err) => match err.error_type {
                    pandaparty_entities::prelude::PandaPartyErrorCode::NotFoundError => not_found!(err),
                    pandaparty_entities::prelude::PandaPartyErrorCode::ExistsAlreadyError => conflict!(err),
                    pandaparty_entities::prelude::PandaPartyErrorCode::InsufficientRightsError => forbidden!(err),
                    pandaparty_entities::prelude::PandaPartyErrorCode::UnauthorizedError => unauthorized!(err),
                    pandaparty_entities::prelude::PandaPartyErrorCode::InvalidDataError | pandaparty_entities::prelude::PandaPartyErrorCode::ValidationError => bad_request!(err),
                    pandaparty_entities::prelude::PandaPartyErrorCode::DbError | pandaparty_entities::prelude::PandaPartyErrorCode::IoError | pandaparty_entities::prelude::PandaPartyErrorCode::SerializationError | pandaparty_entities::prelude::PandaPartyErrorCode::UnknownError => internal_server_error!(err),
                }
            }
        }
    };
}

macro_rules! created_or_error {
    ($data:expr) => {
        {
            match $data {
                Ok(data) => created_json!(data),
                Err(err) => match err.error_type {
                    pandaparty_entities::prelude::PandaPartyErrorCode::NotFoundError => not_found!(err),
                    pandaparty_entities::prelude::PandaPartyErrorCode::ExistsAlreadyError => conflict!(err),
                    pandaparty_entities::prelude::PandaPartyErrorCode::InsufficientRightsError => forbidden!(err),
                    pandaparty_entities::prelude::PandaPartyErrorCode::UnauthorizedError => unauthorized!(err),
                    pandaparty_entities::prelude::PandaPartyErrorCode::InvalidDataError | pandaparty_entities::prelude::PandaPartyErrorCode::ValidationError => bad_request!(err),
                    pandaparty_entities::prelude::PandaPartyErrorCode::DbError | pandaparty_entities::prelude::PandaPartyErrorCode::IoError | pandaparty_entities::prelude::PandaPartyErrorCode::SerializationError | pandaparty_entities::prelude::PandaPartyErrorCode::UnknownError => internal_server_error!(err),
                }
            }
        }
    };
}

macro_rules! ok_json {
    ($data:expr) => {
        {
            actix_web::HttpResponse::Ok().json($data)
        }
    };
}

macro_rules! created_json {
    ($data:expr) => {
        {
            actix_web::HttpResponse::Created().json($data)
        }
    };
}

pub type DbConnection = actix_web::web::Data<DatabaseConnection>;

pub mod routes;
pub mod middleware;
pub mod sse;
pub mod broadcaster;
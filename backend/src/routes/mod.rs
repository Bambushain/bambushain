macro_rules! username {
    ($req:ident) => {
        {
            use actix_web::HttpMessage;
            let extensions = $req.extensions();
            let state = extensions.get::<crate::middleware::authenticate_user::AuthenticationState>().expect("AuthenticationState should be set");
            state.user.username.to_string()
        }
    };
}

macro_rules! no_content {
    () => {
        {
            actix_web::HttpResponse::new(actix_web::http::StatusCode::NO_CONTENT)
        }
    };
}

macro_rules! forbidden {
    () => {
        {
            actix_web::HttpResponse::new(actix_web::http::StatusCode::FORBIDDEN)
        }
    };
}

macro_rules! not_found {
    () => {
        {
            actix_web::HttpResponse::new(actix_web::http::StatusCode::NOT_FOUND)
        }
    };
}

macro_rules! conflict {
    () => {
        {
            actix_web::HttpResponse::new(actix_web::http::StatusCode::CONFLICT)
        }
    };
}

macro_rules! internal_server_error {
    () => {
        {
            actix_web::HttpResponse::new(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    };
}

macro_rules! no_content_or_internal_server_error {
    ($data:expr) => {
        {
            if let Ok(_) = $data {
                no_content!()
            } else {
                internal_server_error!()
            }
        }
    };
}

macro_rules! ok_or_not_found {
    ($data:expr) => {
        {
            if let Some(data) = $data {
                ok_json!(data)
            } else {
                not_found!()
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

pub mod user;
pub mod authentication;
pub mod crafter;
pub mod fighter;
pub mod calendar;
pub mod kill;
pub mod mount;
pub mod savage_mount;
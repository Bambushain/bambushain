macro_rules! check_missing_fields {
    ($body:expr, $entity:expr) => {
        if let Some(body) = $body {
            Ok(body)
        } else {
            Err(bamboo_error::BambooError::invalid_data(
                $entity,
                "There are missing fields",
            ))
        }
    };
}

macro_rules! check_invalid_path {
    ($path:expr, $entity:expr) => {
        if let Some(path) = $path {
            Ok(path)
        } else {
            Err(bamboo_error::BambooError::invalid_data(
                $entity,
                "The path data are invalid",
            ))
        }
    };
}

macro_rules! check_invalid_query {
    ($query:expr, $entity:expr) => {
        if let Some(query) = $query {
            Ok(query)
        } else {
            Err(bamboo_error::BambooError::invalid_data(
                $entity,
                "The query data are invalid",
            ))
        }
    };
}

macro_rules! ok {
    ($data:expr) => {
        ($data, actix_web::http::StatusCode::OK)
    };
}

macro_rules! list {
    ($data:expr) => {
        actix_web::HttpResponse::Ok().json($data)
    };
}

macro_rules! created {
    ($data:expr) => {
        ($data, actix_web::http::StatusCode::CREATED)
    };
}

macro_rules! no_content {
    () => {
        actix_web::HttpResponse::NoContent().finish()
    };
}

pub(crate) use check_invalid_path;
pub(crate) use check_invalid_query;
pub(crate) use check_missing_fields;
pub(crate) use created;
pub(crate) use list;
pub(crate) use no_content;
pub(crate) use ok;

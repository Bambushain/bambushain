use actix_web::{get, web, Responder};
use bamboo_common::backend::dbal;
use bamboo_common::backend::response::*;
use bamboo_common::backend::services::{DbConnection, MinioService};
use bamboo_common::core::error::*;
use serde::Deserialize;

use crate::middleware::authenticate_user::{authenticate, Authentication};
use crate::path;

#[derive(Deserialize)]
struct UsersQuery {
    pub grove: Option<i32>,
}

#[get("/api/user", wrap = "authenticate!()")]
pub async fn get_users(
    query: Option<web::Query<UsersQuery>>,
    db: DbConnection,
    authentication: Authentication,
) -> BambooApiResponseResult {
    let query = check_invalid_query!(query, "user")?;

    if let Some(grove) = query.grove {
        dbal::get_users_by_grove(authentication.user.id, grove.clone(), &db)
            .await
            .map(|data| list!(data))
    } else {
        dbal::get_users(authentication.user.id, &db)
            .await
            .map(|data| list!(data))
    }
}

#[get("/api/user/{user_id}/picture", wrap = "authenticate!()")]
pub async fn get_profile_picture(
    path: Option<path::UserPath>,
    minio: MinioService,
) -> impl Responder {
    if let Ok(path) = check_invalid_path!(path, "user") {
        let profile_picture = minio.get_profile_picture(path.user_id).await;
        if let Ok(profile_picture) = profile_picture {
            return actix_web::HttpResponse::Ok().body(profile_picture);
        }
    }

    actix_web::HttpResponse::Ok()
        .content_type("image/svg+xml")
        .body(bytes::Bytes::from(include_str!(
            "../assets/default-profile-picture.svg"
        )))
}

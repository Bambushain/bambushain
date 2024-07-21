use actix_web::{get, Responder};

use bamboo_common::backend::dbal;
use bamboo_common::backend::response::*;
use bamboo_common::backend::services::{DbConnection, MinioService};
use bamboo_common::core::error::*;

use crate::middleware::authenticate_user::{authenticate, Authentication};
use crate::path;

#[get("/api/user", wrap = "authenticate!()")]
pub async fn get_users(
    db: DbConnection,
    authentication: Authentication,
) -> BambooApiResponseResult {
    dbal::get_users(authentication.user.id, &db)
        .await
        .map(|data| list!(data))
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

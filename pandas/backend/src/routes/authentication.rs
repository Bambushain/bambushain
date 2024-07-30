use actix_web::cookie::Cookie;
use actix_web::{delete, post, web, HttpResponse};
use bamboo_common::backend::actix::cookie;
use bamboo_common::backend::dbal;
use bamboo_common::backend::dbal::create_token;
use bamboo_common::backend::response::*;
use bamboo_common::backend::services::DbConnection;
use bamboo_common::core::entities::*;
use bamboo_common::core::error::*;

use bamboo_common::backend::actix::middleware::{authenticate, Authentication};

#[post("/api/login")]
pub async fn login(body: Option<web::Json<Login>>, db: DbConnection) -> BambooApiResponseResult {
    let body = check_missing_fields!(body, "authentication")?;

    let data = dbal::validate_auth(
        body.email.clone(),
        body.password.clone(),
        body.two_factor_code.clone(),
        &db,
    )
    .await
    .map_err(|err| {
        log::error!("Failed to login {err}");
        BambooError::unauthorized("user", "Login data is invalid")
    })?;

    if data.requires_two_factor_code {
        Ok(no_content!())
    } else {
        create_token(body.email.clone(), &db)
            .await
            .map_err(|err| {
                log::error!("Failed to login {err}");
                BambooError::unauthorized("user", "Login data is invalid")
            })
            .map(|data| {
                let mut response = list!(data.clone());
                let _ = response.add_cookie(
                    &Cookie::build(cookie::BAMBOO_AUTH_COOKIE, data.token.clone())
                        .path("/")
                        .http_only(true)
                        .finish(),
                );

                response
            })
    }
}

#[post("/api/forgot-password")]
pub async fn forgot_password() -> HttpResponse {
    // TODO: Password reset links will be implemented in TG-14
    no_content!()
}

#[delete("/api/login", wrap = "authenticate!()")]
pub async fn logout(auth: Authentication, db: DbConnection) -> HttpResponse {
    let _ = dbal::delete_token(auth.token.clone(), &db).await;

    no_content!()
}

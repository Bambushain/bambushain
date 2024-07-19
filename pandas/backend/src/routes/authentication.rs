use actix_web::cookie::Cookie;
use actix_web::{delete, post, web, HttpResponse};
use bamboo_common::backend::dbal::create_token;
use bamboo_common::backend::response::*;
use bamboo_common::backend::services::{DbConnection, EnvService};
use bamboo_common::backend::{dbal, mailing};
use bamboo_common::core::entities::*;
use bamboo_common::core::error::*;

use crate::middleware::authenticate_user::{authenticate, Authentication};

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
                    &Cookie::build(crate::cookie::BAMBOO_AUTH_COOKIE, data.token.clone())
                        .path("/")
                        .http_only(true)
                        .finish(),
                );

                response
            })
    }
}

#[post("/api/forgot-password")]
pub async fn forgot_password(
    body: Option<web::Json<ForgotPassword>>,
    db: DbConnection,
    env_service: EnvService,
) -> HttpResponse {
    if let Ok(body) = check_missing_fields!(body, "user") {
        if let Ok(user) = dbal::get_user_by_email_or_username(body.email.clone(), &db).await {
            if let Ok(mods) = dbal::get_users_with_mod_rights(user.id, &db).await {
                for bamboo_mod in mods {
                    mailing::authentication::send_forgot_password_mail(
                        user.display_name.clone(),
                        bamboo_mod.display_name.clone(),
                        bamboo_mod.email.clone(),
                        env_service.clone(),
                    )
                    .await
                }
            }
        }
    }

    no_content!()
}

#[delete("/api/login", wrap = "authenticate!()")]
pub async fn logout(auth: Authentication, db: DbConnection) -> HttpResponse {
    let _ = dbal::delete_token(auth.token.clone(), &db).await;

    no_content!()
}

use actix_web::cookie::Cookie;
use actix_web::{delete, post, web, HttpResponse};

use bamboo_common::backend::response::*;
use bamboo_common::backend::services::{DbConnection, EnvService};
use bamboo_common::backend::{dbal, mailing};
use bamboo_common::core::entities::*;
use bamboo_common::core::error::*;

use crate::middleware::authenticate_user::{authenticate, Authentication};

#[post("/api/login")]
pub async fn login(
    body: Option<web::Json<Login>>,
    db: DbConnection,
    env_service: EnvService,
) -> BambooApiResponseResult {
    let body = check_missing_fields!(body, "authentication")?;

    if body.email.clone() == "playstore@google.bambushain" {
        dbal::create_google_auth_token(body.password.clone(), &db)
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
    } else if let Some(two_factor_code) = body.two_factor_code.clone() {
        dbal::validate_auth_and_create_token(
            body.email.clone(),
            body.password.clone(),
            two_factor_code,
            &db,
        )
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
    } else {
        let data = dbal::validate_auth_and_set_two_factor_code(
            body.email.clone(),
            body.password.clone(),
            &db,
        )
        .await
        .map_err(|err| {
            log::error!("Failed to login {err}");
            BambooError::unauthorized("user", "Login data is invalid")
        })?;
        if let Some(two_factor_code) = data.two_factor_code {
            mailing::authentication::send_two_factor_mail(
                data.user.display_name,
                data.user.email,
                two_factor_code,
                env_service,
            )
            .await
            .map(|_| no_content!())
        } else {
            Ok(no_content!())
        }
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

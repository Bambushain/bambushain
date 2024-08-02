use actix_web::cookie::Cookie;
use actix_web::{delete, post, web, HttpResponse};
use bamboo_common::backend::actix::cookie;
use bamboo_common::backend::dbal;
use bamboo_common::backend::dbal::create_token;
use bamboo_common::backend::response::*;
use bamboo_common::backend::services::DbConnection;
use bamboo_common::core::entities::*;
use bamboo_common::core::error::*;
use chrono::Locale;
use maud::html;

use bamboo_common::backend::actix::middleware::{authenticate, Authentication};
use bamboo_common::backend::mailing::{enqueue_mail, Mail};

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
pub async fn forgot_password(
    body: Option<web::Json<ForgotPassword>>,
    db: DbConnection,
) -> BambooApiResponseResult {
    let body = check_missing_fields!(body, "authentication")?.into_inner();
    if let Ok(user) = dbal::get_user_by_email_or_username(body.email, &db).await {
        if let Ok((token, valid_until)) = dbal::set_forgot_password_token(user.id, &db).await {
            let mail_body = html! {
                mj-text {
                    p {
                        (format!("Hey {},", user.display_name))
                    }
                    p {
                        "du willst dein Passwort zurücksetzen?" br;
                        "Falls ja, klick einfach unten auf den Button du kannst dann ein neues Passwort vergeben." br;
                        (format!("Der Link ist bis {} gültig.", valid_until.format_localized("%A den %-d. %B %C%y", Locale::de_DE_euro)))
                    }
                    p {
                       "Bitte beachte, dass deine Zwei Faktor Authentifizierung zurückgesetzt wird."
                    }
                    p {
                        "Alles Gute" br;
                        "Dein Panda Helferlein"
                    }
                }
            }.into_string();

            enqueue_mail(Mail::new_templated(
                "Passwort vergessen",
                user.email,
                mail_body,
                None as Option<String>,
                "Passwort zurücksetzen",
                format!("https://bambushain.app/pandas/forgot-password?token={token}"),
            ))
            .await;
        }
    }

    Ok(no_content!())
}

#[post("/api/reset-password")]
pub async fn reset_password(
    body: Option<web::Json<ResetPassword>>,
    db: DbConnection,
) -> BambooApiResponseResult {
    let body = check_missing_fields!(body, "authentication")?.into_inner();

    dbal::reset_password_by_token(
        body.email.clone(),
        body.token.clone(),
        body.password.clone(),
        &db,
    )
    .await
    .map(|_| no_content!())
}

#[delete("/api/login", wrap = "authenticate!()")]
pub async fn logout(auth: Authentication, db: DbConnection) -> HttpResponse {
    let _ = dbal::delete_token(auth.token.clone(), &db).await;

    no_content!()
}

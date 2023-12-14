use actix_web::{cookie::Cookie, delete, post, web, HttpResponse};

use bamboo_dbal::prelude::*;
use bamboo_entities::prelude::*;
use bamboo_error::*;
use bamboo_services::prelude::{DbConnection, EnvService};

use crate::mailing;
use crate::middleware::authenticate_user::{authenticate, Authentication};
use crate::response::macros::*;

async fn send_two_factor_mail(
    display_name: String,
    to: String,
    token: String,
    env_service: EnvService,
) -> BambooApiResponseResult {
    let env_service = env_service.clone();
    let html_body = format!(
        r#"
<html lang="de" style="font-family: system-ui,-apple-system,'Segoe UI','Roboto','Ubuntu','Cantarell','Noto Sans',sans-serif,'Apple Color Emoji','Segoe UI Emoji','Segoe UI Symbol','Noto Color Emoji';">
<head>

</head>
<body>
    <article style="margin: 4rem 0; padding: 4rem 2rem; border-radius: 0.25rem; background: #fff; box-shadow: 0.0145rem 0.029rem 0.174rem rgba(27, 40, 50, 0.01698),0.0335rem 0.067rem 0.402rem rgba(27, 40, 50, 0.024),0.0625rem 0.125rem 0.75rem rgba(27, 40, 50, 0.03),0.1125rem 0.225rem 1.35rem rgba(27, 40, 50, 0.036),0.2085rem 0.417rem 2.502rem rgba(27, 40, 50, 0.04302),0.5rem 1rem 6rem rgba(27, 40, 50, 0.06),0 0 0 0.0625rem rgba(27, 40, 50, 0.015);">
        Hallo {display_name},<br><br>
        hier ist dein Zwei-Faktor-Code für den Bambushain: <kbd style="background-color: #1b2832; color: #fff; vertical-align: baseline; display: inline-block; padding: .375rem .5rem; border-radius: 0.25rem; font-weight: bolder; line-height: initial; font-size: .875em; font-family: menlo, consolas, 'roboto mono', 'ubuntu monospace','noto mono','oxygen mono','liberation mono',monospace,'apple color emoji','segoe ui symbol','noto emoji'">{token}</kbd><br><br>
        Alles Gute vom 🐼
    </article>
</body>
</html>"#
    );
    let plain_body = format!(
        r#"
Hallo {display_name},

hier ist dein Zwei-Faktor-Code für den Bambushain: {token}

Alles Gute vom 🐼"#
    );

    mailing::send_mail(
        env_service,
        "Dein Zwei-Factor-Code für den Bambushain",
        to,
        plain_body,
        html_body,
    )
    .await
    .map_err(|err| {
        log::error!("Failed to send email {err}");
        log::error!("{err:#?}");

        BambooError::unauthorized("user", "Login data is invalid")
    })
    .map(|_| no_content!())
}

#[post("/api/login")]
pub async fn login(
    body: Option<web::Json<Login>>,
    db: DbConnection,
    env_service: EnvService,
) -> BambooApiResponseResult {
    let body = check_missing_fields!(body, "authentication")?;

    if let Some(two_factor_code) = body.two_factor_code.clone() {
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
            send_two_factor_mail(
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

#[delete("/api/login", wrap = "authenticate!()")]
pub async fn logout(auth: Authentication, db: DbConnection) -> HttpResponse {
    let _ = dbal::delete_token(auth.token.clone(), &db).await;

    no_content!()
}

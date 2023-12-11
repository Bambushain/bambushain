use actix_web::{cookie::Cookie, delete, post, web, HttpResponse};
use lettre::message::MultiPart;
use lettre::transport::smtp;
use lettre::transport::smtp::client::TlsParameters;
use lettre::AsyncTransport;

use bamboo_dbal::prelude::*;
use bamboo_entities::prelude::*;
use bamboo_error::*;
use bamboo_services::prelude::{DbConnection, EnvService};

use crate::middleware::authenticate_user::{authenticate, Authentication};
use crate::response::macros::*;

async fn send_two_factor_mail(
    display_name: String,
    email: String,
    token: String,
    env_service: EnvService,
) -> BambooApiResponseResult {
    let env_service = env_service.clone();
    let html_template = format!(
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
    let text_template = format!(
        r#"
Hallo {display_name},

hier ist dein Zwei-Faktor-Code für den Bambushain: {token}

Alles Gute vom 🐼"#
    );

    let email = lettre::Message::builder()
        .from(
            env_service
                .get_env("MAILER_FROM", "noreply@bambushain.app")
                .parse()
                .unwrap(),
        )
        .to(email.parse().unwrap())
        .subject("Dein Zwei-Factor-Code für den Bambushain")
        .multipart(MultiPart::alternative_plain_html(
            text_template,
            html_template,
        ))
        .map_err(|err| {
            log::error!("Failed to construct the email message {err}");
            BambooError::unauthorized("user", "Login data is invalid")
        })?;

    let mail_server = env_service.get_env("MAILER_SERVER", "localhost");
    let builder = if env_service
        .get_env("MAILER_STARTTLS", "false")
        .to_lowercase()
        == "true"
    {
        lettre::AsyncSmtpTransport::<lettre::Tokio1Executor>::starttls_relay(mail_server.as_str())
    } else {
        lettre::AsyncSmtpTransport::<lettre::Tokio1Executor>::relay(mail_server.as_str())
    }
    .map_err(|err| {
        log::error!("Failed to create the email builder {err}");
        BambooError::unauthorized("user", "Login data is invalid")
    })?;

    let port = env_service
        .get_env("MAILER_PORT", "25")
        .parse::<u16>()
        .unwrap_or(25u16);
    let builder = if env_service.get_env("MAILER_ENCRYPTION", "false") == "false" {
        builder.tls(smtp::client::Tls::None)
    } else {
        builder.tls(smtp::client::Tls::Required(
            TlsParameters::new(mail_server).map_err(|err| {
                log::error!("Failed to parse the server domain {err}");
                BambooError::unauthorized("user", "Login data is invalid")
            })?,
        ))
    };

    let mailer = builder
        .credentials(smtp::authentication::Credentials::new(
            env_service.get_env("MAILER_USERNAME", ""),
            env_service.get_env("MAILER_PASSWORD", ""),
        ))
        .port(port)
        .build();

    mailer
        .send(email)
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

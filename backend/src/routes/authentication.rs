use actix_web::{HttpResponse, web};
use lettre::AsyncTransport;
use lettre::message::MultiPart;
use lettre::transport::smtp;
use lettre::transport::smtp::client::TlsParameters;

use pandaparty_dbal::prelude::*;
use pandaparty_entities::prelude::*;

use crate::{DbConnection, Services};
use crate::middleware::authenticate_user::AuthenticationState;

async fn send_two_factor_mail(display_name: String, email: String, token: String, services: Services) -> HttpResponse {
    let env_service = services.environment_service.clone();
    let html_template = format!(r#"
<html lang="de" style="font-family: system-ui,-apple-system,'Segoe UI','Roboto','Ubuntu','Cantarell','Noto Sans',sans-serif,'Apple Color Emoji','Segoe UI Emoji','Segoe UI Symbol','Noto Color Emoji';">
<head>

</head>
<body>
    <article style="margin: 4rem 0; padding: 4rem 2rem; border-radius: 0.25rem; background: #fff; box-shadow: 0.0145rem 0.029rem 0.174rem rgba(27, 40, 50, 0.01698),0.0335rem 0.067rem 0.402rem rgba(27, 40, 50, 0.024),0.0625rem 0.125rem 0.75rem rgba(27, 40, 50, 0.03),0.1125rem 0.225rem 1.35rem rgba(27, 40, 50, 0.036),0.2085rem 0.417rem 2.502rem rgba(27, 40, 50, 0.04302),0.5rem 1rem 6rem rgba(27, 40, 50, 0.06),0 0 0 0.0625rem rgba(27, 40, 50, 0.015);">
        Hallo {display_name},<br><br>
        hier ist dein Zwei-Faktor-Code: <kbd style="background-color: #1b2832; color: #fff; vertical-align: baseline; display: inline-block; padding: .375rem .5rem; border-radius: 0.25rem; font-weight: bolder; line-height: initial; font-size: .875em; font-family: menlo, consolas, 'roboto mono', 'ubuntu monospace','noto mono','oxygen mono','liberation mono',monospace,'apple color emoji','segoe ui symbol','noto emoji'">{token}</kbd><br><br>
        Alles Gute vom 🐼
    </article>
</body>
</html>"#);
    let text_template = format!(r#"
Hallo {display_name},

hier ist dein Zwei-Faktor-Code: {token}

Alles Gute vom 🐼"#);

    let email = match lettre::Message::builder()
        .from(env_service.get_env("MAILER_FROM", "noreply@creastina.art").parse().unwrap())
        .to(email.parse().unwrap())
        .subject("Dein Zwei-Factor-Code für die Pandaparty")
        .multipart(
            MultiPart::alternative_plain_html(text_template, html_template)
        ) {
        Ok(email) => email,
        Err(err) => {
            log::error!("Failed to construct the email message {err}");
            return HttpResponse::Unauthorized().json(PandaPartyError {
                entity_type: "user".to_string(),
                message: "Email or Password is invalid".to_string(),
                error_type: PandaPartyErrorCode::InvalidDataError,
            });
        }
    };

    let mail_server = env_service.get_env("MAILER_SERVER", "localhost");
    let transport = if env_service.get_env("MAILER_STARTTLS", "false").to_lowercase() == "true" {
        lettre::AsyncSmtpTransport::<lettre::Tokio1Executor>::starttls_relay(mail_server.as_str())
    } else {
        lettre::AsyncSmtpTransport::<lettre::Tokio1Executor>::relay(mail_server.as_str())
    };

    let builder = match transport {
        Ok(builder) => builder,
        Err(err) => {
            log::error!("Failed to create the email builder {err}");
            return HttpResponse::Unauthorized().json(PandaPartyError {
                entity_type: "user".to_string(),
                message: "Email or Password is invalid".to_string(),
                error_type: PandaPartyErrorCode::InvalidDataError,
            });
        }
    };

    let port = env_service.get_env("MAILER_PORT", "25").parse::<u16>().unwrap_or(25u16);
    let builder = if env_service.get_env("MAILER_ENCRYPTION", "false") == "false" {
        builder.tls(smtp::client::Tls::None)
    } else {
        builder.tls(smtp::client::Tls::Required(TlsParameters::new(mail_server).expect("Should work")))
    };

    let mailer = builder
        .credentials(smtp::authentication::Credentials::new(env_service.get_env("MAILER_USERNAME", ""), env_service.get_env("MAILER_PASSWORD", "")))
        .port(port)
        .build();

    match mailer.send(email).await {
        Ok(_) => no_content!(),
        Err(err) => {
            log::error!("Failed to send email {err}");
            log::error!("{err:#?}");
            HttpResponse::Unauthorized().json(PandaPartyError {
                entity_type: "user".to_string(),
                message: "Email or Password is invalid".to_string(),
                error_type: PandaPartyErrorCode::InvalidDataError,
            })
        }
    }
}

pub async fn login(body: web::Json<Login>, db: DbConnection, services: Services) -> HttpResponse {
    if let Some(two_factor_code) = body.two_factor_code.clone() {
        let data = validate_auth_and_create_token(body.email.clone(), body.password.clone(), two_factor_code, &db).await;
        match data {
            Ok(result) => ok_json!(result),
            Err(err) => {
                log::error!("Failed to login {err}");
                HttpResponse::Unauthorized().json(PandaPartyError {
                    entity_type: "user".to_string(),
                    message: "Email or Password is invalid".to_string(),
                    error_type: PandaPartyErrorCode::InvalidDataError,
                })
            }
        }
    } else {
        let data = validate_auth_and_set_two_factor_code(body.email.clone(), body.password.clone(), &db).await;
        match data {
            Ok(result) => send_two_factor_mail(result.user.display_name, result.user.email, result.two_factor_code, services).await,
            Err(err) => {
                log::error!("Failed to login {err}");
                HttpResponse::Unauthorized().json(PandaPartyError {
                    entity_type: "user".to_string(),
                    message: "Email or Password is invalid".to_string(),
                    error_type: PandaPartyErrorCode::InvalidDataError,
                })
            }
        }
    }
}

pub async fn logout(state: web::ReqData<AuthenticationState>, db: DbConnection) -> HttpResponse {
    let _ = delete_token(state.token.clone(), &db).await;

    no_content!()
}
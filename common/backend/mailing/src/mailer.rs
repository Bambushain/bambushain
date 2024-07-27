use bamboo_common_backend_services::EnvService;
use lettre::message::{MessageBuilder, MultiPart};
use lettre::transport::smtp;
use lettre::transport::smtp::client::TlsParameters;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

use bamboo_common_core::error::{BambooError, BambooErrorResult, BambooResult};

fn get_transport(env_service: EnvService) -> BambooResult<AsyncSmtpTransport<Tokio1Executor>> {
    let mail_server = env_service.get_env("MAILER_SERVER", "localhost");
    let builder = if env_service
        .get_env("MAILER_STARTTLS", "false")
        .to_lowercase()
        == "true"
    {
        AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(mail_server.as_str())
    } else {
        AsyncSmtpTransport::<Tokio1Executor>::relay(mail_server.as_str())
    }
    .map_err(|_| BambooError::mailing("Failed to create the email builder"))?;

    let port = env_service
        .get_env("MAILER_PORT", "25")
        .parse::<u16>()
        .unwrap_or(25u16);
    let transport = if env_service.get_env("MAILER_ENCRYPTION", "false") == "false" {
        builder.tls(smtp::client::Tls::None)
    } else {
        builder.tls(smtp::client::Tls::Required(
            TlsParameters::new(mail_server).map_err(|err| {
                log::error!("Failed to parse the server domain {err}");

                BambooError::mailing("Failed to parse the server domain")
            })?,
        ))
    }
    .credentials(smtp::authentication::Credentials::new(
        env_service.get_env("MAILER_USERNAME", ""),
        env_service.get_env("MAILER_PASSWORD", ""),
    ))
    .port(port)
    .build();

    Ok(transport)
}

fn build_message(
    env_service: EnvService,
    subject: impl Into<String>,
    to: impl Into<String>,
) -> MessageBuilder {
    Message::builder()
        .from(
            env_service
                .get_env("MAILER_FROM", "panda.helferlein@bambushain.app")
                .parse()
                .unwrap(),
        )
        .to(to.into().parse().unwrap())
        .subject(subject)
}

pub async fn send_mail(
    env_service: EnvService,
    subject: impl Into<String>,
    to: impl Into<String>,
    plain_body: impl Into<String>,
    html_body: impl Into<String>,
) -> BambooErrorResult {
    let email = build_message(env_service.clone(), subject, to)
        .multipart(MultiPart::alternative_plain_html(
            plain_body.into(),
            html_body.into(),
        ))
        .map_err(|_| BambooError::mailing("Failed to construct the email message"))?;

    get_transport(env_service)?
        .send(email)
        .await
        .map_err(|_| BambooError::mailing("Failed to send email"))
        .map(|_| ())
}

pub async fn send_mail_with_reply_to(
    env_service: EnvService,
    subject: impl Into<String>,
    to: impl Into<String>,
    reply_to: impl Into<String>,
    plain_body: impl Into<String>,
    html_body: impl Into<String>,
) -> BambooErrorResult {
    let email = build_message(env_service.clone(), subject, to)
        .reply_to(reply_to.into().parse().unwrap())
        .multipart(MultiPart::alternative_plain_html(
            plain_body.into(),
            html_body.into(),
        ))
        .map_err(|_| BambooError::mailing("Failed to construct the email message"))?;

    get_transport(env_service)?
        .send(email)
        .await
        .map_err(|_| BambooError::mailing("Failed to send email"))
        .map(|_| ())
}

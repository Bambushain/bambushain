use bamboo_common::backend::mailing::Mail;
use bamboo_common::backend::services::EnvironmentService;
use bamboo_common::core::error::{BambooError, BambooErrorResult, BambooResult};
use lettre::message::MessageBuilder;
use lettre::message::{Mailbox, MultiPart};
use lettre::transport::smtp;
use lettre::transport::smtp::client::TlsParameters;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

fn get_transport(
    env_service: &EnvironmentService,
) -> BambooResult<AsyncSmtpTransport<Tokio1Executor>> {
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
    env_service: &EnvironmentService,
    subject: impl Into<String>,
    to: impl Into<String>,
) -> BambooResult<MessageBuilder> {
    let mbox = Mailbox::new(
        Some("Panda Helferlein".to_string()),
        env_service
            .get_env("MAILER_FROM", "panda.helferlein@bambushain.app")
            .parse()
            .map_err(|_| BambooError::mailing("Failed to parse from address"))?,
    );

    Ok(Message::builder()
        .from(mbox)
        .to(to
            .into()
            .parse()
            .map_err(|_| BambooError::mailing("Failed to parse to address"))?)
        .subject(subject))
}

pub async fn send_mail(mail: Mail, env_service: EnvironmentService) -> BambooErrorResult {
    let plain_body = html2text::config::plain()
        .string_from_read(mail.body.as_bytes(), 40)
        .map_err(|_| BambooError::mailing("Failed to strip html tags"))?;

    let email = if let Some(reply_to) = mail.reply_to {
        build_message(&env_service, mail.subject, mail.to)?.reply_to(
            reply_to
                .parse()
                .map_err(|_| BambooError::mailing("Failed to parse reply to address"))?,
        )
    } else {
        build_message(&env_service, mail.subject, mail.to)?
    }
    .multipart(MultiPart::alternative_plain_html(plain_body, mail.body))
    .map_err(|_| BambooError::mailing("Failed to construct the email message"))?;

    get_transport(&env_service)?
        .send(email)
        .await
        .map_err(|_| BambooError::mailing("Failed to send email"))
        .map(|_| ())
}

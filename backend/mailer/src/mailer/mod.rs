use crate::template;
use bamboo_common::backend::mailing::Mail;
use bamboo_common::backend::services::EnvironmentService;
use bamboo_common::core::error::{BambooError, BambooErrorResult, BambooResult};
use lettre::message::header::ContentType;
use lettre::message::{Attachment, Body, Mailbox, MultiPart};
use lettre::message::{MessageBuilder, SinglePart};
use lettre::transport::smtp;
use lettre::transport::smtp::client::TlsParameters;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use maud::{Markup, PreEscaped};

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

async fn convert_html_body(mail: Mail) -> BambooResult<String> {
    if mail.templated {
        template::mail(
            mail.subject,
            Markup::from(PreEscaped(mail.body)),
            mail.action_label,
            mail.action_link,
        )
        .await
    } else {
        Ok(mail.body)
    }
}

async fn convert_plain_body(mail: Mail) -> BambooResult<String> {
    let body = if mail.templated {
        template::mail(
            mail.subject,
            Markup::from(PreEscaped(mail.body)),
            mail.action_label.map(|res| {
                format!(
                    "{res}: {}",
                    mail.action_link.clone().unwrap_or("".to_string())
                )
            }),
            mail.action_link,
        )
        .await
    } else {
        Ok(mail.body)
    }?;
    html2text::config::rich()
        .string_from_read(body.as_bytes(), 140)
        .map_err(|_| BambooError::mailing("Failed to strip html tags"))
}

pub async fn send_mail(mail: Mail, env_service: EnvironmentService) -> BambooErrorResult {
    let email = if let Some(reply_to) = mail.reply_to.clone() {
        build_message(&env_service, mail.subject.clone(), mail.to.clone())?.reply_to(
            reply_to
                .parse()
                .map_err(|_| BambooError::mailing("Failed to parse reply to address"))?,
        )
    } else {
        build_message(&env_service, mail.subject.clone(), mail.to.clone())?
    }
    .multipart(
        MultiPart::alternative()
            .singlepart(SinglePart::plain(convert_plain_body(mail.clone()).await?))
            .multipart(
                MultiPart::related()
                    .singlepart(SinglePart::html(convert_html_body(mail.clone()).await?))
                    .singlepart(Attachment::new_inline("logo".to_string()).body(
                        Body::new(include_bytes!("logo.gif").to_vec()),
                        ContentType::parse("image/gif").unwrap(),
                    ))
                    .singlepart(Attachment::new_inline("background".to_string()).body(
                        Body::new(include_bytes!("background.jpg").to_vec()),
                        ContentType::parse("image/jpeg").unwrap(),
                    )),
            ),
    )
    .map_err(|_| BambooError::mailing("Failed to construct the email message"))?;

    get_transport(&env_service)?
        .send(email)
        .await
        .map_err(|_| BambooError::mailing("Failed to send email"))
        .map(|_| ())
}

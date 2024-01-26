use bamboo_common_backend_services::EnvService;
use bamboo_common_core::error::{BambooError, BambooErrorResult};

use crate::mailer::send_mail;

pub async fn send_user_created(
    display_name: String,
    created_by: String,
    to: String,
    password: String,
    env_service: EnvService,
) -> BambooErrorResult {
    let env_service = env_service.clone();
    let html_body = format!(
        r#"
<html lang="de" style="font-family: system-ui,-apple-system,'Segoe UI','Roboto','Ubuntu','Cantarell','Noto Sans',sans-serif,'Apple Color Emoji','Segoe UI Emoji','Segoe UI Symbol','Noto Color Emoji';">
<head>

</head>
<body>
    <article style="margin: 4rem 0; padding: 4rem 2rem; border-radius: 0.25rem; background: #fff; box-shadow: 0.0145rem 0.029rem 0.174rem rgba(27, 40, 50, 0.01698),0.0335rem 0.067rem 0.402rem rgba(27, 40, 50, 0.024),0.0625rem 0.125rem 0.75rem rgba(27, 40, 50, 0.03),0.1125rem 0.225rem 1.35rem rgba(27, 40, 50, 0.036),0.2085rem 0.417rem 2.502rem rgba(27, 40, 50, 0.04302),0.5rem 1rem 6rem rgba(27, 40, 50, 0.06),0 0 0 0.0625rem rgba(27, 40, 50, 0.015);">
        Hallo {display_name},<br><br>
        {created_by} hat dir einen Account im Bambushain angelegt, willkommen bei den Pandas, schön das du da bist 🙂
        <br><br>
        Du kannst dich unter <a style="color: #598c79;text-decoration: none" href="https://pandas.bambushain.app">https://pandas.bambushain.app</a> mit der Emailadresse <kbd style="background-color: #1b2832; color: #fff; vertical-align: baseline; display: inline-block; padding: .375rem .5rem; border-radius: 0.25rem; font-weight: bolder; line-height: initial; font-size: .875em; font-family: menlo, consolas, 'roboto mono', 'ubuntu monospace','noto mono','oxygen mono','liberation mono',monospace,'apple color emoji','segoe ui symbol','noto emoji'">{to}</kbd> und dem Passwort <kbd style="background-color: #1b2832; color: #fff; vertical-align: baseline; display: inline-block; padding: .375rem .5rem; border-radius: 0.25rem; font-weight: bolder; line-height: initial; font-size: .875em; font-family: menlo, consolas, 'roboto mono', 'ubuntu monospace','noto mono','oxygen mono','liberation mono',monospace,'apple color emoji','segoe ui symbol','noto emoji'">{password}</kbd> anmelden. Wenn beides korrekt ist wird dir an deine Emailadresse ein Zwei Faktor Code geschickt.<br><br>
        Alles Gute vom 🐼
    </article>
</body>
</html>"#
    );
    let plain_body = format!(
        r#"
Hallo {display_name},

{created_by} hat dir einen Account im Bambushain angelegt, willkommen bei den Pandas, schön das du da bist 🙂

Du kannst dich unter https://pandas.bambushain.app mit der Emailadresse {to} und dem Passwort {password} anmelden. Wenn beides korrekt ist wird dir an deine Emailadresse ein Zwei Faktor Code geschickt.

Alles Gute vom 🐼"#
    );

    send_mail(
        env_service,
        "Willkommen im Bambushain",
        to,
        plain_body,
        html_body,
    )
    .await
    .map_err(|err| {
        log::error!("Failed to send email {err}");
        log::error!("{err:#?}");

        BambooError::mailing("Failed to send welcome email")
    })
}

pub async fn send_password_changed(
    display_name: String,
    to: String,
    password: String,
    app_totp_enabled: bool,
    env_service: EnvService,
) -> BambooErrorResult {
    let env_service = env_service.clone();
    let app_totp_message = if app_totp_enabled {
        " Dein Zwei Faktor Code wird dir wieder per Mail geschickt."
    } else {
        ""
    };
    let html_body = format!(
        r#"
<html lang="de" style="font-family: system-ui,-apple-system,'Segoe UI','Roboto','Ubuntu','Cantarell','Noto Sans',sans-serif,'Apple Color Emoji','Segoe UI Emoji','Segoe UI Symbol','Noto Color Emoji';">
<head>

</head>
<body>
    <article style="margin: 4rem 0; padding: 4rem 2rem; border-radius: 0.25rem; background: #fff; box-shadow: 0.0145rem 0.029rem 0.174rem rgba(27, 40, 50, 0.01698),0.0335rem 0.067rem 0.402rem rgba(27, 40, 50, 0.024),0.0625rem 0.125rem 0.75rem rgba(27, 40, 50, 0.03),0.1125rem 0.225rem 1.35rem rgba(27, 40, 50, 0.036),0.2085rem 0.417rem 2.502rem rgba(27, 40, 50, 0.04302),0.5rem 1rem 6rem rgba(27, 40, 50, 0.06),0 0 0 0.0625rem rgba(27, 40, 50, 0.015);">
        Hallo {display_name},<br><br>
        Dein Passwort wurde zurückgesetzt, hier ist dein neues Passwort <kbd style="background-color: #1b2832; color: #fff; vertical-align: baseline; display: inline-block; padding: .375rem .5rem; border-radius: 0.25rem; font-weight: bolder; line-height: initial; font-size: .875em; font-family: menlo, consolas, 'roboto mono', 'ubuntu monospace','noto mono','oxygen mono','liberation mono',monospace,'apple color emoji','segoe ui symbol','noto emoji'">{password}</kbd>.{app_totp_message}<br><br>
        Alles Gute vom 🐼
    </article>
</body>
</html>"#
    );
    let plain_body = format!(
        r#"
Hallo {display_name},

Dein Passwort wurde zurückgesetzt, hier ist dein neues Passwort {password}.{app_totp_message}

Alles Gute vom 🐼"#
    );

    send_mail(
        env_service,
        "Dein Passwort wurde zurückgesetzt",
        to,
        plain_body,
        html_body,
    )
    .await
    .map_err(|err| {
        log::error!("Failed to send email {err}");
        log::error!("{err:#?}");

        BambooError::mailing("Failed to send change password email")
    })
}

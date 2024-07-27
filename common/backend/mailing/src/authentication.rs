use bamboo_common_backend_services::EnvService;

use crate::mailer::send_mail;

pub async fn send_forgot_password_mail(
    display_name: String,
    mod_name: String,
    to: String,
    env_service: EnvService,
) {
    let env_service = env_service.clone();
    let html_body = format!(
        r#"
<html lang="de" style="font-family: system-ui,-apple-system,'Segoe UI','Roboto','Ubuntu','Cantarell','Noto Sans',sans-serif,'Apple Color Emoji','Segoe UI Emoji','Segoe UI Symbol','Noto Color Emoji';">
<head>

</head>
<body>
    <article style="margin: 4rem 0; padding: 4rem 2rem; border-radius: 0.25rem; background: #fff; box-shadow: 0.0145rem 0.029rem 0.174rem rgba(27, 40, 50, 0.01698),0.0335rem 0.067rem 0.402rem rgba(27, 40, 50, 0.024),0.0625rem 0.125rem 0.75rem rgba(27, 40, 50, 0.03),0.1125rem 0.225rem 1.35rem rgba(27, 40, 50, 0.036),0.2085rem 0.417rem 2.502rem rgba(27, 40, 50, 0.04302),0.5rem 1rem 6rem rgba(27, 40, 50, 0.06),0 0 0 0.0625rem rgba(27, 40, 50, 0.015);">
        Hallo {mod_name},<br><br>
        {display_name} braucht ein neues Passwort, kannst du dich mit den anderen Mods abstimmen, damit ihr euch drum kümmert? Du kannst das Passwort in der Pandasseite zurücksetzen.<br><br>
        Alles Gute vom 🐼
    </article>
</body>
</html>"#
    );
    let plain_body = format!(
        r#"
Hallo {mod_name},

{display_name} braucht ein neues Passwort, kannst du dich mit den anderen Mods abstimmen, damit ihr euch drum kümmert? Du kannst das Passwort in der Pandasseite zurücksetzen.

Alles Gute vom 🐼"#
    );

    let _ = send_mail(
        env_service,
        format!("{display_name} braucht ein neues Passwort"),
        to,
        plain_body,
        html_body,
    )
    .await
    .map_err(|err| {
        log::error!("Failed to send email {err}");
        log::error!("{err:#?}");
    })
    .map(|_| ());
}

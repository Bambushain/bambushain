use actix_web::{delete, get, post, put, web};
use rand::distributions::Alphanumeric;
use rand::Rng;

use bamboo_dbal::prelude::dbal;
use bamboo_entities::prelude::*;
use bamboo_error::*;
use bamboo_services::prelude::{DbConnection, EnvService};

use crate::{mailing, path};
use crate::middleware::authenticate_user::{authenticate, Authentication};
use crate::middleware::check_mod::is_mod;
use crate::middleware::identify_grove::{CurrentGrove, grove};
use crate::response::macros::*;

fn get_random_password() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect::<String>()
}

async fn send_user_created(
    display_name: String,
    created_by: String,
    to: String,
    password: String,
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

    mailing::send_mail(
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
        .map(|_| no_content!())
}

async fn send_password_changed(
    display_name: String,
    to: String,
    password: String,
    app_totp_enabled: bool,
    env_service: EnvService,
) -> BambooApiResponseResult {
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

    mailing::send_mail(
        env_service,
        "Dein Passwort wurde zurückgsetzt",
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
        .map(|_| no_content!())
}

#[get("/api/user", wrap = "authenticate!()", wrap = "grove!()")]
pub async fn get_users(current_grove: CurrentGrove, db: DbConnection) -> BambooApiResponseResult {
    dbal::get_users(current_grove.grove.id, &db)
        .await
        .map(|users| {
            list!(users
                .into_iter()
                .map(WebUser::from)
                .collect::<Vec<WebUser>>())
        })
}

#[get("/api/user/{user_id}", wrap = "authenticate!()", wrap = "grove!()")]
pub async fn get_user(
    path: Option<path::UserPath>,
    current_grove: CurrentGrove,
    db: DbConnection,
) -> BambooApiResult<WebUser> {
    let path = check_invalid_path!(path, "user")?;

    dbal::get_user(current_grove.grove.id, path.user_id, &db)
        .await
        .map(|data| ok!(data.into()))
}

#[post("/api/user", wrap = "authenticate!()", wrap = "is_mod!()", wrap = "grove!()")]
pub async fn create_user(
    body: Option<web::Json<User>>,
    current_grove: CurrentGrove,
    authentication: Authentication,
    env_service: EnvService,
    db: DbConnection,
) -> BambooApiResult<WebUser> {
    let body = check_missing_fields!(body, "user")?;
    let new_password = get_random_password();
    let user = dbal::create_user(current_grove.grove.id, body.into_inner(), new_password.clone(), &db).await?;
    send_user_created(user.display_name.clone(), authentication.user.display_name.clone(), user.email.clone(), new_password, env_service).await?;

    Ok(created!(user.into()))
}

#[delete("/api/user/{user_id}", wrap = "authenticate!()", wrap = "is_mod!()", wrap = "grove!()")]
pub async fn delete_user(
    path: Option<path::UserPath>,
    current_grove: CurrentGrove,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "user")?;
    if path.user_id == authentication.user.id {
        return Err(BambooError::validation(
            "user",
            "You cannot delete yourself",
        ));
    }

    dbal::delete_user(current_grove.grove.id, path.user_id, &db)
        .await
        .map(|_| no_content!())
}

#[put("/api/user/{user_id}/mod", wrap = "authenticate!()", wrap = "is_mod!()", wrap = "grove!()")]
pub async fn add_mod_user(
    path: Option<path::UserPath>,
    current_grove: CurrentGrove,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "user")?;
    if path.user_id == authentication.user.id {
        return Err(BambooError::validation(
            "user",
            "You cannot make yourself mod",
        ));
    }

    dbal::change_mod_status(current_grove.grove.id, path.user_id, true, &db)
        .await
        .map(|_| no_content!())
}

#[delete("/api/user/{user_id}/mod", wrap = "authenticate!()", wrap = "is_mod!()", wrap = "grove!()")]
pub async fn remove_mod_user(
    path: Option<path::UserPath>,
    current_grove: CurrentGrove,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "user")?;
    if path.user_id == authentication.user.id {
        return Err(BambooError::validation(
            "user",
            "You cannot revoke your own mod rights",
        ));
    }

    dbal::change_mod_status(current_grove.grove.id, path.user_id, false, &db)
        .await
        .map(|_| no_content!())
}

#[put("/api/user/{user_id}/password", wrap = "authenticate!()", wrap = "is_mod!()", wrap = "grove!()")]
pub async fn change_password(
    path: Option<path::UserPath>,
    current_grove: CurrentGrove,
    authentication: Authentication,
    env_service: EnvService,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "user")?;
    if path.user_id == authentication.user.id {
        return Err(BambooError::validation(
            "user",
            "You cannot change your own password using this endpoint",
        ));
    }

    let new_password = get_random_password();
    dbal::change_password(
        current_grove.grove.id,
        path.user_id,
        new_password.clone(),
        &db,
    )
        .await?;

    let user = dbal::get_user(current_grove.grove.id, path.user_id, &db).await?;
    send_password_changed(user.display_name.clone(), user.email.clone(), new_password, user.totp_validated.unwrap_or(false), env_service).await
}

#[put("/api/user/{user_id}/profile", wrap = "authenticate!()", wrap = "is_mod!()", wrap = "grove!()")]
pub async fn update_user_profile(
    path: Option<path::UserPath>,
    body: Option<web::Json<UpdateProfile>>,
    current_grove: CurrentGrove,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "user")?;
    let body = check_missing_fields!(body, "user")?;

    dbal::update_profile(
        current_grove.grove.id,
        path.user_id,
        body.email.clone(),
        body.display_name.clone(),
        body.discord_name.clone(),
        &db,
    )
        .await
        .map(|_| no_content!())
}

#[delete("/api/user/{user_id}/totp", wrap = "authenticate!()", wrap = "is_mod!()", wrap = "grove!()")]
pub async fn disable_totp(
    path: Option<path::UserPath>,
    current_grove: CurrentGrove,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "user")?;
    if path.user_id == authentication.user.id {
        return Err(BambooError::validation(
            "user",
            "You cannot disable your own two factor authentication",
        ));
    }

    dbal::disable_totp(current_grove.grove.id, path.user_id, &db)
        .await
        .map(|_| no_content!())
}

use actix_web::{web, HttpResponse};
use serde::Deserialize;

use bamboo_dbal::prelude::*;
use bamboo_entities::prelude::*;
use bamboo_error::*;
use bamboo_services::prelude::DbConnection;

use crate::middleware::authenticate_user::Authentication;

#[derive(Deserialize)]
pub struct UserPathInfo {
    pub id: i32,
}

macro_rules! prevent_me {
    ($me:expr, $passed_user:expr, $error_message:expr) => {{
        if $me == $passed_user {
            return conflict!(bamboo_validation_error!("user", $error_message));
        };
    }};
}

pub async fn get_users(db: DbConnection) -> HttpResponse {
    ok_or_error!(bamboo_dbal::user::get_users(&db).await.map(|users| users
        .into_iter()
        .map(|u| u.to_web_user())
        .collect::<Vec<WebUser>>()))
}

pub async fn get_user(path: Option<web::Path<UserPathInfo>>, db: DbConnection) -> HttpResponse {
    let path = check_invalid_path!(path, "user");

    ok_or_error!(bamboo_dbal::user::get_user(path.id, &db)
        .await
        .map(|u| u.to_web_user()))
}

pub async fn create_user(body: Option<web::Json<User>>, db: DbConnection) -> HttpResponse {
    let body = check_missing_fields!(body, "user");

    if user_exists(body.id, &db).await {
        return conflict!(bamboo_exists_already_error!(
            "user",
            "A user with the name already exists"
        ));
    }

    created_or_error!(bamboo_dbal::user::create_user(body.into_inner(), &db)
        .await
        .map(|u| u.to_web_user()))
}

pub async fn delete_user(
    path: Option<web::Path<UserPathInfo>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    let path = check_invalid_path!(path, "user");

    prevent_me!(
        authentication.user.id,
        path.id,
        "You cannot delete yourself"
    );
    if !user_exists(path.id, &db).await {
        return not_found!(bamboo_not_found_error!("user", "The user was not found"));
    }

    no_content_or_error!(bamboo_dbal::user::delete_user(path.id, &db).await)
}

pub async fn add_mod_user(
    path: Option<web::Path<UserPathInfo>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    let path = check_invalid_path!(path, "user");

    prevent_me!(
        authentication.user.id,
        path.id,
        "You cannot make yourself mod"
    );
    if !user_exists(path.id, &db).await {
        return not_found!(bamboo_not_found_error!("user", "The user was not found"));
    }

    no_content_or_error!(change_mod_status(path.id, true, &db).await)
}

pub async fn remove_mod_user(
    path: Option<web::Path<UserPathInfo>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    let path = check_invalid_path!(path, "user");

    prevent_me!(
        authentication.user.id,
        path.id,
        "You cannot revoke your own mod rights"
    );
    if !user_exists(path.id, &db).await {
        return not_found!(bamboo_not_found_error!("user", "The user was not found"));
    }

    no_content_or_error!(change_mod_status(path.id, false, &db).await)
}

pub async fn change_password(
    path: Option<web::Path<UserPathInfo>>,
    body: Option<web::Json<ChangePassword>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    let path = check_invalid_path!(path, "user");
    let body = check_missing_fields!(body, "user");

    prevent_me!(
        authentication.user.id,
        path.id,
        "You cannot change your own password using this endpoint"
    );
    if !user_exists(path.id, &db).await {
        return not_found!(bamboo_not_found_error!("user", "The user was not found"));
    }

    no_content_or_error!(
        bamboo_dbal::user::change_password(path.id, body.new_password.clone(), &db).await
    )
}

pub async fn change_my_password(
    body: Option<web::Json<ChangeMyPassword>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    let body = check_missing_fields!(body, "user");

    match bamboo_dbal::user::change_my_password(
        authentication.user.id,
        body.old_password.clone(),
        body.new_password.clone(),
        &db,
    )
    .await
    {
        Ok(_) => no_content!(),
        Err(PasswordError::WrongPassword) => forbidden!(bamboo_insufficient_rights_error!(
            "user",
            "The current password is wrong"
        )),
        Err(PasswordError::UserNotFound) => {
            not_found!(bamboo_not_found_error!("user", "The user was not found"))
        }
        Err(PasswordError::UnknownError) => {
            internal_server_error!(bamboo_unknown_error!("user", "An unknown error occurred"))
        }
    }
}

pub async fn update_profile(
    body: Option<web::Json<UpdateProfile>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    let body = check_missing_fields!(body, "user");

    no_content_or_error!(
        update_me(
            authentication.user.id,
            body.email.clone(),
            body.display_name.clone(),
            body.discord_name.clone(),
            &db,
        )
        .await
    )
}

pub async fn update_user_profile(
    path: Option<web::Path<UserPathInfo>>,
    body: Option<web::Json<UpdateProfile>>,
    db: DbConnection,
) -> HttpResponse {
    let path = check_invalid_path!(path, "user");
    let body = check_missing_fields!(body, "user");

    no_content_or_error!(
        update_me(
            path.id,
            body.email.clone(),
            body.display_name.clone(),
            body.discord_name.clone(),
            &db,
        )
        .await
    )
}

pub async fn get_profile(authentication: Authentication) -> HttpResponse {
    ok_json!(authentication.user.to_web_user())
}

pub async fn enable_totp(authentication: Authentication, db: DbConnection) -> HttpResponse {
    let mut totp = totp_rs::TOTP::default();
    let secret = totp.secret.clone();
    let data = bamboo_dbal::user::enable_totp(authentication.user.id, secret, &db).await;

    match data {
        Ok(_) => {
            totp.account_name = authentication.user.display_name.clone();
            totp.issuer = Some("Bambushain".to_string());
            let qr = match totp.get_qr_base64() {
                Ok(qr) => qr,
                Err(err) => {
                    log::error!("Failed to enable totp {err}");
                    let _ = disable_totp(authentication.user.id, &db).await;
                    return internal_server_error!();
                }
            };

            ok_json!(TotpQrCode {
                qr_code: qr,
                secret: totp.get_secret_base32(),
            })
        }
        Err(err) => {
            log::error!("Failed to enable totp {err}");
            internal_server_error!()
        }
    }
}

pub async fn validate_totp(
    body: Option<web::Json<ValidateTotp>>,
    authentication: Authentication,
    db: DbConnection,
) -> HttpResponse {
    if authentication.user.totp_validated.unwrap_or(false) {
        bad_request!(bamboo_invalid_data_error!("user", "Already validated"))
    } else {
        let body = check_missing_fields!(body, "user");

        let result = bamboo_dbal::user::validate_totp(
            authentication.user.id,
            body.password.clone(),
            body.code.clone(),
            &db,
        )
        .await;

        match result {
            Ok(true) => no_content!(),
            Ok(false) => forbidden!(),
            Err(err) => error_response!(err),
        }
    }
}

use actix_web::{delete, get, post, put, web};

use bamboo_dbal::prelude::dbal;
use bamboo_entities::prelude::*;
use bamboo_error::*;
use bamboo_services::prelude::DbConnection;

use crate::middleware::authenticate_user::{authenticate, Authentication};
use crate::middleware::check_mod::is_mod;
use crate::middleware::identify_grove::{grove, CurrentGrove};
use crate::path;
use crate::response::macros::*;

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

#[post(
    "/api/user",
    wrap = "authenticate!()",
    wrap = "is_mod!()",
    wrap = "grove!()"
)]
pub async fn create_user(
    body: Option<web::Json<User>>,
    current_grove: CurrentGrove,
    db: DbConnection,
) -> BambooApiResult<WebUser> {
    let body = check_missing_fields!(body, "user")?;

    dbal::create_user(current_grove.grove.id, body.into_inner(), &db)
        .await
        .map(|data| created!(data.into()))
}

#[delete(
    "/api/user/{user_id}",
    wrap = "authenticate!()",
    wrap = "is_mod!()",
    wrap = "grove!()"
)]
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

#[put(
    "/api/user/{user_id}/mod",
    wrap = "authenticate!()",
    wrap = "is_mod!()",
    wrap = "grove!()"
)]
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

#[delete(
    "/api/user/{user_id}/mod",
    wrap = "authenticate!()",
    wrap = "is_mod!()",
    wrap = "grove!()"
)]
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

#[put(
    "/api/user/{user_id}/password",
    wrap = "authenticate!()",
    wrap = "is_mod!()",
    wrap = "grove!()"
)]
pub async fn change_password(
    path: Option<path::UserPath>,
    body: Option<web::Json<ChangePassword>>,
    current_grove: CurrentGrove,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "user")?;
    let body = check_missing_fields!(body, "user")?;
    if path.user_id == authentication.user.id {
        return Err(BambooError::validation(
            "user",
            "You cannot change your own password using this endpoint",
        ));
    }

    dbal::change_password(
        current_grove.grove.id,
        path.user_id,
        body.new_password.clone(),
        &db,
    )
    .await
    .map(|_| no_content!())
}

#[put(
    "/api/user/{user_id}/profile",
    wrap = "authenticate!()",
    wrap = "is_mod!()",
    wrap = "grove!()"
)]
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

#[delete(
    "/api/user/{user_id}/totp",
    wrap = "authenticate!()",
    wrap = "is_mod!()",
    wrap = "grove!()"
)]
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

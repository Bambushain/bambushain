use actix_web::{delete, get, post, put, web};

use bamboo_dbal::prelude::dbal;
use bamboo_entities::prelude::*;
use bamboo_error::*;
use bamboo_services::prelude::DbConnection;

use crate::middleware::authenticate_user::{authenticate, Authentication};
use crate::middleware::check_mod::is_mod;
use crate::path;
use crate::response::macros::*;

#[get("/api/user", wrap = "authenticate!()")]
pub async fn get_users(db: DbConnection) -> BambooApiResponseResult {
    dbal::get_users(&db).await.map(|users| {
        list!(users
            .into_iter()
            .map(WebUser::from)
            .collect::<Vec<WebUser>>())
    })
}

#[get("/api/user/{user_id}", wrap = "authenticate!()")]
pub async fn get_user(path: Option<path::UserPath>, db: DbConnection) -> BambooApiResult<WebUser> {
    let path = check_invalid_path!(path, "user")?;

    dbal::get_user(path.user_id, &db)
        .await
        .map(|data| ok!(data.into()))
}

#[post("/api/user", wrap = "authenticate!()", wrap = "is_mod!()")]
pub async fn create_user(
    body: Option<web::Json<User>>,
    db: DbConnection,
) -> BambooApiResult<WebUser> {
    let body = check_missing_fields!(body, "user")?;

    dbal::create_user(body.into_inner(), &db)
        .await
        .map(|data| created!(data.into()))
}

#[delete("/api/user/{user_id}", wrap = "authenticate!()", wrap = "is_mod!()")]
pub async fn delete_user(
    path: Option<path::UserPath>,
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

    dbal::delete_user(path.user_id, &db).await.map(|_| no_content!())
}

#[put(
    "/api/user/{user_id}/mod",
    wrap = "authenticate!()",
    wrap = "is_mod!()"
)]
pub async fn add_mod_user(
    path: Option<path::UserPath>,
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

    dbal::change_mod_status(path.user_id, true, &db)
        .await
        .map(|_| no_content!())
}

#[delete(
    "/api/user/{user_id}/mod",
    wrap = "authenticate!()",
    wrap = "is_mod!()"
)]
pub async fn remove_mod_user(
    path: Option<path::UserPath>,
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

    dbal::change_mod_status(path.user_id, false, &db)
        .await
        .map(|_| no_content!())
}

#[put(
    "/api/user/{user_id}/password",
    wrap = "authenticate!()",
    wrap = "is_mod!()"
)]
pub async fn change_password(
    path: Option<path::UserPath>,
    body: Option<web::Json<ChangePassword>>,
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

    dbal::change_password(path.user_id, body.new_password.clone(), &db)
        .await
        .map(|_| no_content!())
}

#[put(
    "/api/user/{user_id}/profile",
    wrap = "authenticate!()",
    wrap = "is_mod!()"
)]
pub async fn update_user_profile(
    path: Option<path::UserPath>,
    body: Option<web::Json<UpdateProfile>>,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "user")?;
    let body = check_missing_fields!(body, "user")?;

    dbal::update_me(
        path.user_id,
        body.email.clone(),
        body.display_name.clone(),
        body.discord_name.clone(),
        &db,
    )
    .await
    .map(|_| no_content!())
}

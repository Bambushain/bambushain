use actix_web::{delete, get, put, web};

use bamboo_common::backend::response::{check_invalid_path, list, no_content};
use bamboo_common::backend::services::{DbConnection, EnvService};
use bamboo_common::backend::utils::get_random_password;
use bamboo_common::backend::{dbal, mailing};
use bamboo_common::core::entities::GroveUser;
use bamboo_common::core::error::{BambooApiResponseResult, BambooError};

use crate::middleware::authenticate_user::authenticate;
use crate::path::{GrovePath, GroveUserPath};

#[get("/api/grove/{grove_id}/user", wrap = "authenticate!()")]
pub async fn get_users(
    path: Option<web::Path<GrovePath>>,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "user")?;
    dbal::get_users_filtered_for_management(path.grove_id, &db)
        .await
        .map(|data| {
            list!(data
                .into_iter()
                .map(GroveUser::from)
                .collect::<Vec<GroveUser>>())
        })
}

#[put(
    "/api/grove/{grove_id}/user/{user_id}/password",
    wrap = "authenticate!()"
)]
pub async fn reset_user_password(
    path: Option<web::Path<GroveUserPath>>,
    db: DbConnection,
    env_service: EnvService,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "user")?;
    let user = dbal::get_user(path.grove_id, path.user_id, &db).await?;

    if user.is_mod {
        let password = get_random_password();
        dbal::change_password(path.grove_id, path.user_id, password.clone(), &db).await?;
        mailing::user::send_password_changed(
            user.display_name.clone(),
            user.email.clone(),
            password.clone(),
            user.totp_validated.unwrap_or(false),
            env_service,
        )
        .await
        .map(|_| no_content!())
    } else {
        Err(BambooError::insufficient_rights(
            "user",
            "Only mods passwords can be reset",
        ))
    }
}

#[put("/api/grove/{grove_id}/user/{user_id}/mod", wrap = "authenticate!()")]
pub async fn make_user_mod(
    path: Option<web::Path<GroveUserPath>>,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "user")?;
    dbal::change_mod_status(path.grove_id, path.user_id, true, &db)
        .await
        .map(|_| no_content!())
}

#[delete("/api/grove/{grove_id}/user/{user_id}/mod", wrap = "authenticate!()")]
pub async fn remove_user_mod(
    path: Option<web::Path<GroveUserPath>>,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "user")?;
    dbal::change_mod_status(path.grove_id, path.user_id, false, &db)
        .await
        .map(|_| no_content!())
}

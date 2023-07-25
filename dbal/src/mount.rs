use sea_orm::{IntoActiveModel, NotSet, QueryOrder};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::*;

use sheef_entities::{mount, mount_to_user, sheef_db_error, user};
use sheef_entities::prelude::*;

pub async fn get_mount(mount: String) -> SheefResult<Mount> {
    let db = open_db_connection!();

    match mount::Entity::find()
        .filter(mount::Column::Name.eq(mount))
        .one(&db)
        .await {
        Ok(Some(mount)) => Ok(mount),
        Ok(None) => Err(sheef_not_found_error!("mount", "Mount was not found")),
        Err(_) => Err(sheef_db_error!("mount", "Failed to load mount"))
    }
}

pub async fn mount_exists(mount: String) -> bool {
    get_mount(mount).await.is_ok()
}

pub async fn activate_mount_for_user(mount: String, username: String) -> SheefErrorResult {
    let db = open_db_connection!();
    let user = get_user_by_username!(username);
    let mount = match get_mount(mount).await {
        Ok(mount) => mount,
        Err(_) => return Err(sheef_not_found_error!("mount", "Mount was not found"))
    };

    match mount_to_user::Entity::find()
        .filter(mount_to_user::Column::MountId.eq(mount.id))
        .filter(mount_to_user::Column::UserId.eq(user.id))
        .one(&db)
        .await {
        Ok(Some(ktu)) => {
            let mut ktu = ktu.into_active_model();
            ktu.user_id = NotSet;
            ktu.mount_id = NotSet;
            ktu
        }
        _ => {
            mount_to_user::ActiveModel {
                user_id: Set(user.id),
                mount_id: Set(mount.id),
            }
        }
    }.save(&db)
        .await
        .map_err(|_| sheef_db_error!("mount", "Failed to create mount for user"))
        .map(|_| ())
}

pub async fn deactivate_mount_for_user(mount: String, username: String) -> SheefErrorResult {
    let db = open_db_connection!();
    let user = get_user_by_username!(username);
    let mount = match get_mount(mount).await {
        Ok(mount) => mount,
        Err(_) => return Err(sheef_not_found_error!("mount", "Mount was not found"))
    };

    mount_to_user::Entity::delete_many()
        .filter(mount_to_user::Column::MountId.eq(mount.id))
        .filter(mount_to_user::Column::UserId.eq(user.id))
        .exec(&db)
        .await
        .map_err(|_| sheef_db_error!("mount", "Failed to remove mount from user"))
        .map(|_| ())
}

pub async fn delete_mount(mount: String) -> SheefErrorResult {
    let db = open_db_connection!();

    mount::Entity::delete_many()
        .filter(mount::Column::Name.eq(mount))
        .exec(&db)
        .await
        .map_err(|_| sheef_db_error!("mount", "Failed to delete mount"))
        .map(|_| ())
}

pub async fn create_mount(mount: Mount) -> SheefResult<Mount> {
    let db = open_db_connection!();

    let model = mount.into_active_model();
    model
        .insert(&db)
        .await
        .map_err(|_| sheef_db_error!("mount", "Failed to create mount"))
}

pub async fn update_mount(mount: String, name: String) -> SheefErrorResult {
    let db = open_db_connection!();

    let mut model = match get_mount(mount).await {
        Ok(mount) => mount.into_active_model(),
        Err(err) => return Err(err)
    };

    model.name = Set(name);
    model
        .update(&db)
        .await
        .map_err(|_| sheef_db_error!("mount", "Failed to update mount"))
        .map(|_| ())
}

pub async fn get_mounts() -> SheefResult<Vec<Mount>> {
    let db = open_db_connection!();

    match mount::Entity::find()
        .order_by_asc(mount::Column::Name)
        .all(&db)
        .await {
        Ok(mounts) => Ok(mounts),
        Err(_) => Err(sheef_db_error!("mount", "Failed to load mounts"))
    }
}

pub async fn get_users_for_mount(mount: String) -> SheefResult<Vec<String>> {
    let db = open_db_connection!();

    match user::Entity::find()
        .order_by_asc(user::Column::Username)
        .inner_join(mount::Entity)
        .filter(mount::Column::Name.eq(mount))
        .all(&db)
        .await {
        Ok(users) => Ok(users.iter().map(|user| user.username.clone()).collect()),
        Err(_) => Err(sheef_db_error!("mount", "Failed to load mounts"))
    }
}

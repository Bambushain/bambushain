use sea_orm::{IntoActiveModel, NotSet, QueryOrder};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::*;

use sheef_entities::{savage_mount, savage_mount_to_user, sheef_db_error, user};
use sheef_entities::prelude::*;

pub async fn get_savage_mount(savage_mount: String) -> SheefResult<SavageMount> {
    let db = open_db_connection!();

    match savage_mount::Entity::find()
        .filter(savage_mount::Column::Name.eq(savage_mount))
        .one(&db)
        .await {
        Ok(Some(savage_mount)) => Ok(savage_mount),
        Ok(None) => Err(sheef_not_found_error!("mount", "Savage mount was not found")),
        Err(_) => Err(sheef_db_error!("mount", "Failed to load savage mount"))
    }
}

pub async fn savage_mount_exists(savage_mount: String) -> bool {
    get_savage_mount(savage_mount).await.is_ok()
}

pub async fn activate_savage_mount_for_user(savage_mount: String, username: String) -> SheefErrorResult {
    let db = open_db_connection!();
    let user = get_user_by_username!(username);
    let savage_mount = match get_savage_mount(savage_mount).await {
        Ok(savage_mount) => savage_mount,
        Err(_) => return Err(sheef_not_found_error!("mount", "Savage mount was not found"))
    };

    match savage_mount_to_user::Entity::find()
        .filter(savage_mount_to_user::Column::SavageMountId.eq(savage_mount.id))
        .filter(savage_mount_to_user::Column::UserId.eq(user.id))
        .one(&db)
        .await {
        Ok(Some(ktu)) => {
            let mut ktu = ktu.into_active_model();
            ktu.user_id = NotSet;
            ktu.savage_mount_id = NotSet;
            ktu
        }
        _ => {
            savage_mount_to_user::ActiveModel {
                user_id: Set(user.id),
                savage_mount_id: Set(savage_mount.id),
            }
        }
    }.save(&db)
        .await
        .map_err(|_| sheef_db_error!("mount", "Failed to create savage mount for user"))
        .map(|_| ())
}

pub async fn deactivate_savage_mount_for_user(mount: String, username: String) -> SheefErrorResult {
    let db = open_db_connection!();
    let user = get_user_by_username!(username);
    let mount = match get_savage_mount(mount).await {
        Ok(mount) => mount,
        Err(_) => return Err(sheef_not_found_error!("mount", "Savage mount was not found"))
    };

    savage_mount_to_user::Entity::delete_many()
        .filter(savage_mount_to_user::Column::SavageMountId.eq(mount.id))
        .filter(savage_mount_to_user::Column::UserId.eq(user.id))
        .exec(&db)
        .await
        .map_err(|_| sheef_db_error!("mount", "Failed to remove savage mount from user"))
        .map(|_| ())
}

pub async fn delete_savage_mount(mount: String) -> SheefErrorResult {
    let db = open_db_connection!();

    savage_mount::Entity::delete_many()
        .filter(savage_mount::Column::Name.eq(mount))
        .exec(&db)
        .await
        .map_err(|_| sheef_db_error!("mount", "Failed to delete savage mount"))
        .map(|_| ())
}

pub async fn create_savage_mount(savage_mount: SavageMount) -> SheefResult<SavageMount> {
    let db = open_db_connection!();

    let model = savage_mount.into_active_model();
    model
        .insert(&db)
        .await
        .map_err(|_| sheef_db_error!("mount", "Failed to create savage mount"))
}

pub async fn update_savage_mount(savage_mount: String, name: String) -> SheefErrorResult {
    let db = open_db_connection!();

    let mut model = match get_savage_mount(savage_mount).await {
        Ok(savage_mount) => savage_mount.into_active_model(),
        Err(err) => return Err(err)
    };

    model.name = Set(name);
    model
        .update(&db)
        .await
        .map_err(|_| sheef_db_error!("mount", "Failed to update savage mount"))
        .map(|_| ())
}

pub async fn get_savage_mounts() -> SheefResult<Vec<SavageMount>> {
    let db = open_db_connection!();

    match savage_mount::Entity::find()
        .order_by_asc(savage_mount::Column::Name)
        .all(&db)
        .await {
        Ok(mounts) => Ok(mounts),
        Err(_) => Err(sheef_db_error!("mount", "Failed to load savage mounts"))
    }
}

pub async fn get_users_for_savage_mount(savage_mount: String) -> SheefResult<Vec<String>> {
    let db = open_db_connection!();

    match user::Entity::find()
        .order_by_asc(user::Column::Username)
        .inner_join(savage_mount::Entity)
        .filter(savage_mount::Column::Name.eq(savage_mount))
        .all(&db)
        .await {
        Ok(users) => Ok(users.iter().map(|user| user.username.clone()).collect()),
        Err(_) => Err(sheef_db_error!("mount", "Failed to load savage mounts"))
    }
}

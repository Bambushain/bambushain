use std::collections::BTreeMap;

use sea_orm::{IntoActiveModel, NotSet};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::*;

use pandaparty_entities::{savage_mount, savage_mount_to_user, pandaparty_db_error, user};
use pandaparty_entities::prelude::*;
use crate::user::get_user;

pub async fn get_savage_mount(savage_mount: String) -> SheefResult<SavageMount> {
    let db = open_db_connection!();

    match savage_mount::Entity::find()
        .filter(savage_mount::Column::Name.eq(savage_mount))
        .one(&db)
        .await {
        Ok(Some(savage_mount)) => Ok(savage_mount),
        Ok(None) => Err(pandaparty_not_found_error!("savage-mount", "Savage mount was not found")),
        Err(err) => {
            log::error!("{err}");
            Err(pandaparty_db_error!("savage-mount", "Failed to load savage mount"))
        }
    }
}

pub async fn savage_mount_exists(savage_mount: String) -> bool {
    get_savage_mount(savage_mount).await.is_ok()
}

pub async fn activate_savage_mount_for_user(savage_mount: String, username: String) -> SheefErrorResult {
    let user = match get_user(username.clone()).await {
        Ok(user) => user,
        Err(err) => {
            log::error!("Failed to load user {}: {err}", username);
            return Err(err);
        }
    };
    let savage_mount = match get_savage_mount(savage_mount).await {
        Ok(savage_mount) => savage_mount,
        Err(_) => return Err(pandaparty_not_found_error!("savage-mount", "Savage mount was not found"))
    };

    let db = open_db_connection!();
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
    }
        .save(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("savage-mount", "Failed to create savage mount for user")
        })
        .map(|_| ())
}

pub async fn deactivate_savage_mount_for_user(savage_mount: String, username: String) -> SheefErrorResult {
    let user = match get_user(username.clone()).await {
        Ok(user) => user,
        Err(err) => {
            log::error!("Failed to load user {}: {err}", username);
            return Err(err);
        }
    };
    let savage_mount = match get_savage_mount(savage_mount).await {
        Ok(savage_mount) => savage_mount,
        Err(_) => return Err(pandaparty_not_found_error!("savage-mount", "Savage mount was not found"))
    };

    let db = open_db_connection!();
    savage_mount_to_user::Entity::delete_many()
        .filter(savage_mount_to_user::Column::SavageMountId.eq(savage_mount.id))
        .filter(savage_mount_to_user::Column::UserId.eq(user.id))
        .exec(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("savage-mount", "Failed to remove savage mount from user")
        })
        .map(|_| ())
}

pub async fn delete_savage_mount(savage_mount: String) -> SheefErrorResult {
    let db = open_db_connection!();

    savage_mount::Entity::delete_many()
        .filter(savage_mount::Column::Name.eq(savage_mount))
        .exec(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("savage-mount", "Failed to delete savage mount")
        })
        .map(|_| ())
}

pub async fn create_savage_mount(savage_mount: SavageMount) -> SheefResult<SavageMount> {
    let mut model = savage_mount.into_active_model();
    model.id = NotSet;

    let db = open_db_connection!();
    model
        .insert(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("savage-mount", "Failed to create savage mount")
        })
}

pub async fn update_savage_mount(savage_mount: String, name: String) -> SheefErrorResult {
    let mut model = match get_savage_mount(savage_mount).await {
        Ok(savage_mount) => savage_mount.into_active_model(),
        Err(err) => return Err(err)
    };

    model.name = Set(name);

    let db = open_db_connection!();
    model
        .update(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("savage-mount", "Failed to update savage mount")
        })
        .map(|_| ())
}

pub async fn get_savage_mounts() -> SheefResult<BTreeMap<String, Vec<String>>> {
    let db = open_db_connection!();

    let data = match savage_mount::Entity::find()
        .find_with_related(user::Entity)
        .all(&db)
        .await {
        Ok(result) => result,
        Err(err) => {
            log::error!("{err}");
            return Err(pandaparty_db_error!("savage-mount", "Failed to load savage mounts"));
        }
    };

    let mut result = BTreeMap::new();
    for (savage_mount, users) in data {
        result.insert(savage_mount.name, users.iter().map(|user| user.username.clone()).collect());
    }

    Ok(result)
}

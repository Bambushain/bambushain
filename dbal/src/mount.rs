use std::collections::BTreeMap;

use sea_orm::{IntoActiveModel, NotSet};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::*;

use sheef_entities::{mount, mount_to_user, sheef_db_error, user};
use sheef_entities::prelude::*;

pub async fn get_mount(mount: String) -> SheefResult<Mount> {
    let db = open_db_connection!();

    let result = match mount::Entity::find()
        .filter(mount::Column::Name.eq(mount))
        .one(&db)
        .await {
        Ok(Some(mount)) => Ok(mount),
        Ok(None) => Err(sheef_not_found_error!("mount", "Mount was not found")),
        Err(err) => {
            log::error!("{err}");
            Err(sheef_db_error!("mount", "Failed to load mount"))
        }
    };

    let _ = db.close().await;

    result
}

pub async fn mount_exists(mount: String) -> bool {
    get_mount(mount).await.is_ok()
}

pub async fn activate_mount_for_user(mount: String, username: String) -> SheefErrorResult {
    let db = open_db_connection!();
    let user = match crate::user::get_user(username.clone()).await {
        Ok(user) => user,
        Err(err) => {
            log::error!("Failed to load user {}: {err}", username);
            return Err(err);
        }
    };
    let mount = match get_mount(mount).await {
        Ok(mount) => mount,
        Err(_) => return Err(sheef_not_found_error!("mount", "Mount was not found"))
    };

    let result = match mount_to_user::Entity::find()
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
        .map_err(|err| {
            log::error!("{err}");
            sheef_db_error!("mount", "Failed to create mount for user")
        })
        .map(|_| ());

    let _ = db.close().await;

    result
}

pub async fn deactivate_mount_for_user(mount: String, username: String) -> SheefErrorResult {
    let db = open_db_connection!();
    let user = match crate::user::get_user(username.clone()).await {
        Ok(user) => user,
        Err(err) => {
            log::error!("Failed to load user {}: {err}", username);
            return Err(err);
        }
    };
    let mount = match get_mount(mount).await {
        Ok(mount) => mount,
        Err(_) => return Err(sheef_not_found_error!("mount", "Mount was not found"))
    };

    let result = mount_to_user::Entity::delete_many()
        .filter(mount_to_user::Column::MountId.eq(mount.id))
        .filter(mount_to_user::Column::UserId.eq(user.id))
        .exec(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            sheef_db_error!("mount", "Failed to remove mount from user")
        })
        .map(|_| ());

    let _ = db.close().await;

    result
}

pub async fn delete_mount(mount: String) -> SheefErrorResult {
    let db = open_db_connection!();

    let result = mount::Entity::delete_many()
        .filter(mount::Column::Name.eq(mount))
        .exec(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            sheef_db_error!("mount", "Failed to delete mount")
        })
        .map(|_| ());

    let _ = db.close().await;

    result
}

pub async fn create_mount(mount: Mount) -> SheefResult<Mount> {
    let db = open_db_connection!();

    let mut model = mount.into_active_model();
    model.id = NotSet;
    let result = model
        .insert(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            sheef_db_error!("mount", "Failed to create mount")
        });

    let _ = db.close().await;

    result
}

pub async fn update_mount(mount: String, name: String) -> SheefErrorResult {
    let db = open_db_connection!();

    let mut model = match get_mount(mount).await {
        Ok(mount) => mount.into_active_model(),
        Err(err) => return Err(err)
    };

    model.name = Set(name);
    let result = model
        .update(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            sheef_db_error!("mount", "Failed to update mount")
        })
        .map(|_| ());

    let _ = db.close().await;

    result
}

pub async fn get_mounts() -> SheefResult<BTreeMap<String, Vec<String>>> {
    let db = open_db_connection!();

    let data = match mount::Entity::find().find_with_related(user::Entity).all(&db).await {
        Ok(result) => result,
        Err(err) => {
            log::error!("{err}");
            return Err(sheef_db_error!("mount", "Failed to load mounts"));
        }
    };

    let _ = db.close().await;

    let mut result = BTreeMap::new();
    for (mount, users) in data {
        result.insert(mount.name, users.iter().map(|user| user.username.clone()).collect());
    }

    Ok(result)
}

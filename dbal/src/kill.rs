use std::collections::BTreeMap;

use sea_orm::{IntoActiveModel, NotSet};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::*;

use sheef_entities::{kill, kill_to_user, sheef_db_error, user};
use sheef_entities::prelude::*;
use crate::user::get_user;

pub async fn get_kill(kill: String) -> SheefResult<Kill> {
    let db = open_db_connection!();

    match kill::Entity::find()
        .filter(kill::Column::Name.eq(kill))
        .one(&db)
        .await {
        Ok(Some(kill)) => Ok(kill),
        Ok(None) => Err(sheef_not_found_error!("kill", "Kill was not found")),
        Err(err) => {
            log::error!("{err}");
            Err(sheef_db_error!("kill", "Failed to load kill"))
        }
    }
}

pub async fn kill_exists(kill: String) -> bool {
    get_kill(kill).await.is_ok()
}

pub async fn activate_kill_for_user(kill: String, username: String) -> SheefErrorResult {
    let user = match get_user(username.clone()).await {
        Ok(user) => user,
        Err(err) => {
            log::error!("Failed to load user {}: {err}", username);
            return Err(err);
        }
    };
    let kill = match get_kill(kill).await {
        Ok(kill) => kill,
        Err(_) => return Err(sheef_not_found_error!("kill", "Kill was not found"))
    };

    let db = open_db_connection!();
    match kill_to_user::Entity::find()
        .filter(kill_to_user::Column::KillId.eq(kill.id))
        .filter(kill_to_user::Column::UserId.eq(user.id))
        .one(&db)
        .await {
        Ok(Some(ktu)) => {
            let mut ktu = ktu.into_active_model();
            ktu.user_id = NotSet;
            ktu.kill_id = NotSet;
            ktu
        }
        _ => {
            kill_to_user::ActiveModel {
                user_id: Set(user.id),
                kill_id: Set(kill.id),
            }
        }
    }
        .save(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            sheef_db_error!("kill", "Failed to create kill for user")
        })
        .map(|_| ())
}

pub async fn deactivate_kill_for_user(kill: String, username: String) -> SheefErrorResult {
    let user = match get_user(username.clone()).await {
        Ok(user) => user,
        Err(err) => {
            log::error!("Failed to load user {}: {err}", username);
            return Err(err);
        }
    };
    let kill = match get_kill(kill).await {
        Ok(kill) => kill,
        Err(_) => return Err(sheef_not_found_error!("kill", "Kill was not found"))
    };

    let db = open_db_connection!();
    kill_to_user::Entity::delete_many()
        .filter(kill_to_user::Column::KillId.eq(kill.id))
        .filter(kill_to_user::Column::UserId.eq(user.id))
        .exec(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            sheef_db_error!("kill", "Failed to remove kill from user")
        })
        .map(|_| ())
}

pub async fn delete_kill(kill: String) -> SheefErrorResult {
    let db = open_db_connection!();

    kill::Entity::delete_many()
        .filter(kill::Column::Name.eq(kill))
        .exec(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            sheef_db_error!("kill", "Failed to delete kill")
        })
        .map(|_| ())
}

pub async fn create_kill(kill: Kill) -> SheefResult<Kill> {
    let mut model = kill.into_active_model();
    model.id = NotSet;

    let db = open_db_connection!();
    model
        .insert(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            sheef_db_error!("kill", "Failed to create kill")
        })
}

pub async fn update_kill(kill: String, name: String) -> SheefErrorResult {
    let mut model = match get_kill(kill).await {
        Ok(kill) => kill.into_active_model(),
        Err(err) => return Err(err)
    };

    model.name = Set(name);

    let db = open_db_connection!();
    model
        .update(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            sheef_db_error!("kill", "Failed to update kill")
        })
        .map(|_| ())
}

pub async fn get_kills() -> SheefResult<BTreeMap<String, Vec<String>>> {
    let db = open_db_connection!();

    let data = match kill::Entity::find()
        .find_with_related(user::Entity)
        .all(&db)
        .await {
        Ok(result) => result,
        Err(err) => {
            log::error!("{err}");
            return Err(sheef_db_error!("kill", "Failed to load kills"));
        }
    };

    let mut result = BTreeMap::new();
    for (kill, users) in data {
        result.insert(kill.name, users.iter().map(|user| user.username.clone()).collect());
    }

    Ok(result)
}

use sea_orm::{IntoActiveModel, NotSet, QueryOrder};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::*;

use sheef_entities::{kill, kill_to_user, sheef_db_error, user};
use sheef_entities::prelude::*;

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
    let db = open_db_connection!();
    let user = get_user_by_username!(username);
    let kill = match get_kill(kill).await {
        Ok(kill) => kill,
        Err(_) => return Err(sheef_not_found_error!("kill", "Kill was not found"))
    };

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
    }.save(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            sheef_db_error!("kill", "Failed to create kill for user")
        })
        .map(|_| ())
}

pub async fn deactivate_kill_for_user(kill: String, username: String) -> SheefErrorResult {
    let db = open_db_connection!();
    let user = get_user_by_username!(username);
    let kill = match get_kill(kill).await {
        Ok(kill) => kill,
        Err(_) => return Err(sheef_not_found_error!("kill", "Kill was not found"))
    };

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
    let db = open_db_connection!();

    let mut model = kill.into_active_model();
    model.id = NotSet;
    model
        .insert(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            sheef_db_error!("kill", "Failed to create kill")
        })
}

pub async fn update_kill(kill: String, name: String) -> SheefErrorResult {
    let db = open_db_connection!();

    let mut model = match get_kill(kill).await {
        Ok(kill) => kill.into_active_model(),
        Err(err) => return Err(err)
    };

    model.name = Set(name);
    model
        .update(&db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            sheef_db_error!("kill", "Failed to update kill")
        })
        .map(|_| ())
}

pub async fn get_kills() -> SheefResult<Vec<Kill>> {
    let db = open_db_connection!();

    match kill::Entity::find()
        .order_by_asc(kill::Column::Name)
        .all(&db)
        .await {
        Ok(kills) => Ok(kills),
        Err(err) => {
            log::error!("{err}");
            Err(sheef_db_error!("kill", "Failed to load kills"))
        }
    }
}

pub async fn get_users_for_kill(kill: String) -> SheefResult<Vec<String>> {
    let db = open_db_connection!();

    match user::Entity::find()
        .order_by_asc(user::Column::Username)
        .inner_join(kill::Entity)
        .filter(kill::Column::Name.eq(kill))
        .all(&db)
        .await {
        Ok(users) => Ok(users.iter().map(|user| user.username.clone()).collect()),
        Err(err) => {
            log::error!("{err}");
            Err(sheef_db_error!("kill", "Failed to load kills"))
        }
    }
}

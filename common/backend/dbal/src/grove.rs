use bamboo_common_core::entities::user::JoinStatus;
use bamboo_common_core::entities::*;
use bamboo_common_core::error::*;
use sea_orm::prelude::*;
use sea_orm::sea_query::Keyword::Null;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    Condition, IntoActiveModel, NotSet, QueryOrder, QuerySelect, TransactionError, TransactionTrait,
};
use sha2::{Digest, Sha512_224};

pub async fn is_grove_mod(
    grove_id: i32,
    user_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<bool> {
    grove_user::Entity::find()
        .filter(
            Condition::all()
                .add(grove_user::Column::UserId.eq(user_id))
                .add(grove_user::Column::GroveId.eq(grove_id))
                .add(grove_user::Column::IsMod.eq(true))
                .add(grove_user::Column::IsBanned.eq(false)),
        )
        .count(db)
        .await
        .map_err(|_| BambooError::unknown("grove", "Failed to find mod"))
        .map(|count| count > 0)
}

pub async fn get_grove(id: i32, user_id: i32, db: &DatabaseConnection) -> BambooResult<Grove> {
    grove::Entity::find_by_id(id)
        .inner_join(grove_user::Entity)
        .filter(grove_user::Column::UserId.eq(user_id))
        .one(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("grove", "Failed to execute database query")
        })?
        .ok_or(BambooError::not_found("grove", "The grove was not found"))
}

pub async fn grove_exists_by_name(name: String, db: &DatabaseConnection) -> BambooResult<bool> {
    grove::Entity::find()
        .filter(grove::Column::Name.eq(name))
        .count(db)
        .await
        .map(|count| count > 0)
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("grove", "Failed to execute database query")
        })
}

pub async fn grove_exists_by_id(
    id: i32,
    name: String,
    db: &DatabaseConnection,
) -> BambooResult<bool> {
    grove::Entity::find()
        .filter(grove::Column::Id.ne(id))
        .filter(grove::Column::Name.eq(name))
        .count(db)
        .await
        .map(|count| count > 0)
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("grove", "Failed to execute database query")
        })
}

pub async fn get_groves(user_id: i32, db: &DatabaseConnection) -> BambooResult<Vec<Grove>> {
    grove::Entity::find()
        .inner_join(grove_user::Entity)
        .filter(grove_user::Column::UserId.eq(user_id))
        .order_by_asc(grove::Column::Id)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("user", "Failed to execute database query")
        })
}

pub async fn get_all_groves(db: &DatabaseConnection) -> BambooResult<Vec<Grove>> {
    grove::Entity::find()
        .order_by_asc(grove::Column::Id)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("user", "Failed to execute database query")
        })
}

pub async fn create_grove(
    name: String,
    invite_active: bool,
    user_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<Grove> {
    let mut active_model = Grove::new(name, invite_active).into_active_model();
    active_model.id = NotSet;

    let grove = active_model.insert(db).await.map_err(|err| {
        log::error!("Failed to create grove {err}");
        BambooError::database("grove", "Failed to create grove")
    })?;

    let mut user = GroveUser::default().into_active_model();
    user.user_id = Set(user_id);
    user.grove_id = Set(grove.id);
    user.is_mod = Set(true);

    user.insert(db).await.map_err(|err| {
        log::error!("Failed to create grove user {err}");
        BambooError::database("grove", "Failed to create grove user")
    })?;

    Ok(grove)
}

pub async fn update_grove(
    id: i32,
    user_id: i32,
    name: String,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    let mut grove = get_grove(id, user_id, db).await?.into_active_model();
    grove.name = Set(name);
    grove
        .update(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("grove", "Failed to update grove")
        })
        .map(|_| ())
}

pub async fn update_grove_mods(
    id: i32,
    user_id: i32,
    mods: Vec<i32>,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    db.transaction(move |tx| {
        Box::pin(async move {
            grove_user::Entity::update_many()
                .filter(grove_user::Column::GroveId.eq(id))
                .filter(grove_user::Column::UserId.ne(user_id))
                .col_expr(grove_user::Column::IsMod, Expr::value(false))
                .exec(tx)
                .await
                .map_err(|err| {
                    log::error!("{err}");
                    BambooError::database("grove", "Failed to update grove mods")
                })?;

            grove_user::Entity::update_many()
                .filter(grove_user::Column::GroveId.eq(id))
                .filter(grove_user::Column::UserId.is_in(mods))
                .col_expr(grove_user::Column::IsMod, Expr::value(true))
                .exec(tx)
                .await
                .map_err(|err| {
                    log::error!("{err}");
                    BambooError::database("grove", "Failed to update grove mods")
                })?;

            Ok(())
        })
    })
    .await
    .map_err(|err: TransactionError<BambooError>| {
        log::error!("{err}");
        BambooError::database("grove", "Failed to update grove mods")
    })
    .map(|_| ())
}

pub async fn delete_grove(id: i32, user_id: i32, db: &DatabaseConnection) -> BambooErrorResult {
    get_grove(id, user_id, db)
        .await?
        .delete(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("grove", "Failed to delete grove")
        })
        .map(|_| ())
}

pub async fn ban_user_from_grove(
    grove_id: i32,
    user_id: i32,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    grove_user::Entity::update_many()
        .filter(
            Condition::all()
                .add(grove_user::Column::UserId.eq(user_id))
                .add(grove_user::Column::GroveId.eq(grove_id))
                .add(grove_user::Column::IsBanned.eq(false)),
        )
        .col_expr(grove_user::Column::IsBanned, Expr::value(true))
        .col_expr(grove_user::Column::IsMod, Expr::value(false))
        .exec(db)
        .await
        .map_err(|_| BambooError::unknown("grove", "Failed to ban user"))
        .map(|_| ())
}

pub async fn unban_user_from_grove(
    grove_id: i32,
    user_id: i32,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    grove_user::Entity::delete_many()
        .filter(
            Condition::all()
                .add(grove_user::Column::UserId.eq(user_id))
                .add(grove_user::Column::GroveId.eq(grove_id))
                .add(grove_user::Column::IsBanned.eq(true)),
        )
        .exec(db)
        .await
        .map_err(|_| BambooError::unknown("grove", "Failed to unban user"))
        .map(|_| ())
}

pub async fn enable_grove_invite(grove_id: i32, db: &DatabaseConnection) -> BambooErrorResult {
    let mut hasher = Sha512_224::new();
    hasher.update(uuid::Uuid::new_v4());
    let res = hasher.finalize();

    grove::Entity::update_many()
        .filter(Condition::all().add(grove::Column::Id.eq(grove_id)))
        .col_expr(
            grove::Column::InviteSecret,
            Expr::value(hex::encode(&res[..10])),
        )
        .exec(db)
        .await
        .map_err(|_| BambooError::unknown("grove", "Failed to enable invites"))
        .map(|_| ())
}

pub async fn disable_grove_invite(grove_id: i32, db: &DatabaseConnection) -> BambooErrorResult {
    grove::Entity::update_many()
        .filter(Condition::all().add(grove::Column::Id.eq(grove_id)))
        .col_expr(grove::Column::InviteSecret, Expr::value(Null))
        .exec(db)
        .await
        .map_err(|_| BambooError::unknown("grove", "Failed to disable invites"))
        .map(|_| ())
}

pub async fn check_grove_join_status(
    grove_id: i32,
    user_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<JoinStatus> {
    grove_user::Entity::find()
        .select_only()
        .column(grove_user::Column::IsBanned)
        .filter(grove_user::Column::UserId.eq(user_id))
        .filter(grove_user::Column::GroveId.eq(grove_id))
        .into_tuple::<bool>()
        .one(db)
        .await
        .map_err(|_| BambooError::unknown("grove", "Join status cannot be checked"))
        .map(|res| {
            if let Some(true) = res {
                JoinStatus::Banned
            } else if let Some(false) = res {
                JoinStatus::Joined
            } else {
                JoinStatus::NotJoined
            }
        })
}

pub async fn join_grove(
    grove_id: i32,
    user_id: i32,
    invite_secret: String,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    grove::Entity::find_by_id(grove_id)
        .select_only()
        .column(grove::Column::InviteSecret)
        .into_tuple::<String>()
        .one(db)
        .await
        .map_err(|_| BambooError::unknown("grove", "Failed to join grove"))
        .map(|secret| {
            if let Some(secret) = secret {
                if secret != invite_secret {
                    Err(BambooError::insufficient_rights(
                        "grove",
                        "The invite secret is wrong",
                    ))
                } else {
                    Ok(())
                }
            } else {
                Err(BambooError::not_found("grove", "The grove was not found"))
            }
        })??;

    match check_grove_join_status(grove_id, user_id, db).await? {
        JoinStatus::Joined => Err(BambooError::exists_already(
            "grove",
            "You joined this grove already",
        )),
        JoinStatus::NotJoined => Ok(()),
        JoinStatus::Banned => Err(BambooError::insufficient_rights(
            "grove",
            "You are banned from this grove",
        )),
    }?;

    grove_user::ActiveModel {
        is_mod: Set(false),
        is_banned: Set(false),
        grove_id: Set(grove_id),
        user_id: Set(user_id),
    }
    .insert(db)
    .await
    .map_err(|_| BambooError::unknown("grove", "Failed to join grove"))
    .map(|_| ())
}

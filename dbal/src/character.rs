use sea_orm::{IntoActiveModel, NotSet, QueryOrder, QuerySelect};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::*;
use sea_orm::sea_query::{Alias, Expr};

use pandaparty_entities::{character, pandaparty_db_error};
use pandaparty_entities::prelude::*;

pub async fn get_characters(user_id: i32, db: &DatabaseConnection) -> PandaPartyResult<Vec<Character>> {
    character::Entity::find()
        .filter(character::Column::UserId.eq(user_id))
        .order_by_asc(character::Column::Name)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("character", "Failed to load characters")
        })
}

pub async fn get_character(id: i32, user_id: i32, db: &DatabaseConnection) -> PandaPartyResult<Character> {
    match character::Entity::find_by_id(id)
        .filter(character::Column::UserId.eq(user_id))
        .one(db)
        .await {
        Ok(Some(res)) => Ok(res),
        Ok(None) => Err(pandaparty_not_found_error!("character", "The character was not found")),
        Err(err) => {
            log::error!("{err}");
            Err(pandaparty_db_error!("character", "Failed to execute database query"))
        }
    }
}

pub async fn character_exists(id: i32, user_id: i32, db: &DatabaseConnection) -> bool {
    match character::Entity::find_by_id(id)
        .filter(character::Column::UserId.eq(user_id))
        .select_only()
        .column(character::Column::Id)
        .count(db)
        .await {
        Ok(count) => count > 0,
        _ => false
    }
}

pub async fn character_exists_by_name(name: String, user_id: i32, db: &DatabaseConnection) -> bool {
    match character::Entity::find()
        .filter(character::Column::Name.eq(name))
        .filter(character::Column::UserId.eq(user_id))
        .select_only()
        .column(character::Column::Id)
        .count(db)
        .await {
        Ok(count) => count > 0,
        _ => false
    }
}

pub async fn create_character(user_id: i32, character: Character, db: &DatabaseConnection) -> PandaPartyResult<Character> {
    let mut model = character.into_active_model();
    model.user_id = Set(user_id);
    model.id = NotSet;

    model
        .insert(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("character", "Failed to create character")
        })
}

pub async fn update_character(id: i32, user_id: i32, character: Character, db: &DatabaseConnection) -> PandaPartyErrorResult {
    character::Entity::update_many()
        .filter(character::Column::Id.eq(id))
        .filter(character::Column::UserId.eq(user_id))
        .col_expr(character::Column::Name, Expr::value(character.name))
        .col_expr(character::Column::World, Expr::value(character.world))
        .col_expr(character::Column::Race, Expr::val(character.race).as_enum(Alias::new("character_race")))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("character", "Failed to update character")
        })
        .map(|_| ())
}

pub async fn delete_character(id: i32, user_id: i32, db: &DatabaseConnection) -> PandaPartyErrorResult {
    character::Entity::delete_many()
        .filter(character::Column::Id.eq(id))
        .filter(character::Column::UserId.eq(user_id))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("character", "Failed to delete character")
        })
        .map(|_| ())
}

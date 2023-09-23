use std::collections::{BTreeMap, BTreeSet};
use sea_orm::prelude::*;
use sea_orm::sea_query::Expr;
use sea_orm::ActiveValue::Set;
use sea_orm::{Condition, IntoActiveModel, NotSet, QueryOrder, QuerySelect};

use pandaparty_entities::prelude::*;
use pandaparty_entities::{
    character, custom_character_field, custom_character_field_option, custom_character_field_value,
    pandaparty_db_error,
};

async fn map_character(
    character: Character,
    user_id: i32,
    db: &DatabaseConnection,
) -> PandaPartyResult<Character> {
    Ok(Character {
        id: character.id,
        race: character.race,
        name: character.name.clone(),
        world: character.world.clone(),
        user_id,
        custom_fields: fill_custom_fields(user_id, character.id, db).await?,
    })
}

pub async fn get_characters(
    user_id: i32,
    db: &DatabaseConnection,
) -> PandaPartyResult<Vec<Character>> {
    let characters = character::Entity::find()
        .filter(character::Column::UserId.eq(user_id))
        .order_by_asc(character::Column::Name)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("character", "Failed to load characters")
        })?;

    let mut result = vec![];
    for character in characters {
        result.push(map_character(character.clone(), user_id, db).await?);
    }

    Ok(result)
}

pub async fn get_character(
    id: i32,
    user_id: i32,
    db: &DatabaseConnection,
) -> PandaPartyResult<Character> {
    let character = character::Entity::find_by_id(id)
        .filter(character::Column::UserId.eq(user_id))
        .one(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("character", "Failed to execute database query")
        })?;

    if let Some(character) = character.into_iter().next() {
        map_character(character, user_id, db).await
    } else {
        Err(pandaparty_not_found_error!(
            "character",
            "The character was not found"
        ))
    }
}

async fn fill_custom_fields(
    user_id: i32,
    character_id: i32,
    db: &DatabaseConnection,
) -> PandaPartyResult<Vec<CustomField>> {
    let data = custom_character_field_value::Entity::find()
        .select_only()
        .inner_join(custom_character_field_option::Entity)
        .inner_join(custom_character_field::Entity)
        .column_as(custom_character_field::Column::Label, "label")
        .column_as(custom_character_field_option::Column::Label, "value")
        .filter(custom_character_field::Column::UserId.eq(user_id))
        .filter(custom_character_field_value::Column::CharacterId.eq(character_id))
        .order_by_asc(custom_character_field::Column::Position)
        .order_by_asc(custom_character_field::Column::Label)
        .into_tuple::<(String, String)>()
        .all(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("character", "Failed to load custom fields")
        })?;

    let positions_from_db = custom_character_field_value::Entity::find()
        .select_only()
        .inner_join(custom_character_field::Entity)
        .column_as(custom_character_field::Column::Label, "label")
        .filter(custom_character_field::Column::UserId.eq(user_id))
        .filter(custom_character_field_value::Column::CharacterId.eq(character_id))
        .order_by_asc(custom_character_field::Column::Position)
        .order_by_asc(custom_character_field::Column::Label)
        .distinct_on(vec![custom_character_field::Column::Label, custom_character_field::Column::Position])
        .into_tuple::<String>()
        .all(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("character", "Failed to load custom fields")
        })?;

    let mut positions = BTreeMap::new();
    for (idx, label) in positions_from_db.iter().enumerate() {
        positions.insert(label.clone(), idx);
    }

    let mut custom_fields: BTreeMap<usize, (String, BTreeSet<String>)> = BTreeMap::new();
    for (label, value) in data {
        let position = positions[&label];
        let values = if custom_fields.contains_key(&position) {
            let (_, mut values) = custom_fields[&position].clone();
            values.insert(value.clone());
            values
        } else {
            vec![value.clone()].into_iter().collect::<BTreeSet<String>>()
        };
        custom_fields.insert(position, (label.clone(), values.clone()));
    }

    Ok(custom_fields
        .into_iter()
        .map(|(position, (label, values))| CustomField { label, values, position })
        .collect::<Vec<CustomField>>())
}

pub async fn character_exists(user_id: i32, id: i32, db: &DatabaseConnection) -> bool {
    character::Entity::find_by_id(id)
        .filter(character::Column::UserId.eq(user_id))
        .select_only()
        .column(character::Column::Id)
        .count(db)
        .await
        .map(|count| count > 0)
        .unwrap_or(false)
}

pub async fn character_exists_by_name(name: String, user_id: i32, db: &DatabaseConnection) -> bool {
    character::Entity::find()
        .filter(character::Column::Name.eq(name))
        .filter(character::Column::UserId.eq(user_id))
        .select_only()
        .column(character::Column::Id)
        .count(db)
        .await
        .map(|count| count > 0)
        .unwrap_or(false)
}

pub async fn create_character(
    user_id: i32,
    character: Character,
    db: &DatabaseConnection,
) -> PandaPartyResult<Character> {
    let mut model = character.clone().into_active_model();
    model.user_id = Set(user_id);
    model.id = NotSet;

    let model = model.insert(db).await.map_err(|err| {
        log::error!("{err}");
        pandaparty_db_error!("character", "Failed to create character")
    })?;

    create_custom_field_values(user_id, model.id, character.custom_fields, db).await?;

    Ok(model)
}

pub async fn update_character(
    id: i32,
    user_id: i32,
    character: Character,
    db: &DatabaseConnection,
) -> PandaPartyErrorResult {
    character::Entity::update_many()
        .filter(character::Column::Id.eq(id))
        .filter(character::Column::UserId.eq(user_id))
        .col_expr(character::Column::Name, Expr::value(character.name.clone()))
        .col_expr(
            character::Column::World,
            Expr::value(character.world.clone()),
        )
        .col_expr(
            character::Column::Race,
            Expr::val(character.race).as_enum(pandaparty_entities::character::CharacterRaceEnum),
        )
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("character", "Failed to update character")
        })?;

    create_custom_field_values(user_id, id, character.custom_fields, db).await
}

async fn create_custom_field_values(
    user_id: i32,
    character_id: i32,
    custom_fields: Vec<CustomField>,
    db: &DatabaseConnection,
) -> PandaPartyErrorResult {
    if custom_fields.is_empty() {
        return Ok(());
    }

    let mut condition = Condition::any();
    for option in custom_fields.iter().flat_map(|field| field.values.clone()) {
        condition = condition.add(custom_character_field_option::Column::Label.eq(option));
    }

    custom_character_field_value::Entity::delete_many()
        .filter(custom_character_field_value::Column::CharacterId.eq(character_id))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("character", "Failed to set custom fields")
        })?;

    let custom_fields = custom_character_field::Entity::find()
        .find_with_related(custom_character_field_option::Entity)
        .filter(custom_character_field::Column::UserId.eq(user_id))
        .filter(condition)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("character", "Failed to set custom fields")
        })?;

    let mut values = vec![];
    for (field, options) in custom_fields {
        for option in options {
            values.push(custom_character_field_value::ActiveModel {
                id: NotSet,
                character_id: Set(character_id),
                custom_character_field_id: Set(field.id),
                custom_character_field_option_id: Set(option.id),
            })
        }
    }

    custom_character_field_value::Entity::insert_many(values)
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("character", "Failed to set custom fields")
        })
        .map(|_| ())
}

pub async fn delete_character(
    id: i32,
    user_id: i32,
    db: &DatabaseConnection,
) -> PandaPartyErrorResult {
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

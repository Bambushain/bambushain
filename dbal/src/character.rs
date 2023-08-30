use std::collections::BTreeMap;

use sea_orm::{Condition, IntoActiveModel, NotSet, QueryOrder, QuerySelect};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::*;
use sea_orm::sea_query::{Alias, Expr};

use pandaparty_entities::{character, custom_character_field, custom_character_field_option, custom_character_field_value, pandaparty_db_error};
use pandaparty_entities::prelude::*;

pub async fn get_characters(user_id: i32, db: &DatabaseConnection) -> PandaPartyResult<Vec<Character>> {
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
        let mut c = character.clone();
        c.custom_fields = fill_custom_fields(user_id, character.id, db).await?;
        result.push(c);
    }

    Ok(result)
}

pub async fn get_character(id: i32, user_id: i32, db: &DatabaseConnection) -> PandaPartyResult<Character> {
    let character = character::Entity::find_by_id(id)
        .filter(character::Column::UserId.eq(user_id))
        .one(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("character", "Failed to execute database query")
        })?;

    if let Some(character) = character.into_iter().next() {
        let mut c = character.clone();
        c.custom_fields = fill_custom_fields(user_id, id, db).await?;
        Ok(c)
    } else {
        Err(pandaparty_not_found_error!("character", "The character was not found"))
    }
}

async fn fill_custom_fields(user_id: i32, character_id: i32, db: &DatabaseConnection) -> PandaPartyResult<Vec<CustomField>> {
    let data = custom_character_field_value::Entity::find()
        .select_only()
        .inner_join(custom_character_field_option::Entity)
        .inner_join(custom_character_field::Entity)
        .column_as(custom_character_field::Column::Label, "label")
        .column_as(custom_character_field_option::Column::Label, "value")
        .filter(custom_character_field::Column::UserId.eq(user_id))
        .filter(custom_character_field_value::Column::CharacterId.eq(character_id))
        .into_tuple::<(String, String)>()
        .all(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("character", "Failed to load custom fields")
        })?;

    let mut custom_fields: BTreeMap<String, CustomField> = BTreeMap::new();
    for (label, value) in data {
        let field = if custom_fields.contains_key(&label) {
            let mut field = custom_fields[&label].clone();
            field.values.push(value);
            field
        } else {
            CustomField {
                values: vec![value],
                label: label.clone(),
            }
        };
        custom_fields.insert(label, field);
    }

    Ok(custom_fields.values().cloned().collect::<Vec<CustomField>>())
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

pub async fn create_character(user_id: i32, character: Character, db: &DatabaseConnection) -> PandaPartyResult<Character> {
    let mut model = character.clone().into_active_model();
    model.user_id = Set(user_id);
    model.id = NotSet;

    let model = model
        .insert(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("character", "Failed to create character")
        })?;

    create_custom_field_values(user_id, character.id, character.custom_fields, db).await?;

    Ok(model)
}

pub async fn update_character(id: i32, user_id: i32, character: Character, db: &DatabaseConnection) -> PandaPartyErrorResult {
    character::Entity::update_many()
        .filter(character::Column::Id.eq(id))
        .filter(character::Column::UserId.eq(user_id))
        .col_expr(character::Column::Name, Expr::value(character.name.clone()))
        .col_expr(character::Column::World, Expr::value(character.world.clone()))
        .col_expr(character::Column::Race, Expr::val(character.race.clone()).as_enum(Alias::new("character_race")))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("character", "Failed to update character")
        })?;

    create_custom_field_values(user_id, id, character.custom_fields, db).await
}

async fn create_custom_field_values(user_id: i32, character_id: i32, custom_fields: Vec<CustomField>, db: &DatabaseConnection) -> PandaPartyErrorResult {
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

pub async fn get_custom_fields(user_id: i32, db: &DatabaseConnection) -> PandaPartyResult<Vec<CustomCharacterField>> {
    let fields = custom_character_field::Entity::find()
        .filter(custom_character_field::Column::UserId.eq(user_id))
        .inner_join(custom_character_field_option::Entity)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("character", "Failed to load character custom fields")
        })?;

    let mut result = vec![];

    for field in fields {
        let options = match custom_character_field_option::Entity::find()
            .filter(custom_character_field::Column::Id.eq(field.id))
            .all(db)
            .await {
            Ok(options) => options,
            Err(err) => {
                log::warn!("Failed to load options: {err}");
                continue;
            }
        };
        let mut f = field.clone();
        f.options = options;
        result.push(f);
    }

    Ok(result)
}

pub async fn get_custom_field(custom_field_id: i32, user_id: i32, db: &DatabaseConnection) -> PandaPartyResult<CustomCharacterField> {
    let field = custom_character_field::Entity::find_by_id(custom_field_id)
        .filter(custom_character_field::Column::UserId.eq(user_id))
        .inner_join(custom_character_field_option::Entity)
        .one(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("character", "Failed to load character custom fields")
        })?;

    if let Some(mut field) = field {
        let options = custom_character_field_option::Entity::find()
            .filter(custom_character_field::Column::Id.eq(field.id))
            .all(db)
            .await
            .map_err(|err| {
                log::error!("{err}");
                pandaparty_db_error!("character", "Failed to load character custom fields")
            })?;
        field.options = options;

        Ok(field)
    } else {
        Err(pandaparty_not_found_error!("character", "Custom field not found"))
    }
}

pub async fn create_custom_field(user_id: i32, custom_field: CustomField, db: &DatabaseConnection) -> PandaPartyResult<CustomCharacterField> {
    let result = custom_character_field::ActiveModel {
        id: NotSet,
        label: Set(custom_field.label),
        user_id: Set(user_id),
    }
        .insert(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("character", "Failed to create custom field")
        })?;

    for value in custom_field.values {
        custom_character_field_option::ActiveModel {
            id: NotSet,
            custom_character_field_id: Set(result.id),
            label: Set(value),
        }
            .insert(db)
            .await
            .map_err(|err| {
                log::error!("{err}");
                pandaparty_db_error!("character", "Failed to create custom field option")
            })?;
    }

    Ok(result)
}

pub async fn update_custom_field(id: i32, user_id: i32, custom_field: CustomField, db: &DatabaseConnection) -> PandaPartyErrorResult {
    custom_character_field::Entity::update_many()
        .filter(custom_character_field::Column::Id.eq(id))
        .filter(custom_character_field::Column::UserId.eq(user_id))
        .col_expr(custom_character_field::Column::Label, Expr::value(custom_field.label))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("character", "Failed to update custom field")
        })
        .map(|_| ())
}

pub async fn delete_custom_field(id: i32, user_id: i32, db: &DatabaseConnection) -> PandaPartyErrorResult {
    custom_character_field::Entity::delete_many()
        .filter(custom_character_field::Column::Id.eq(id))
        .filter(custom_character_field::Column::UserId.eq(user_id))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("character", "Failed to delete custom field")
        })
        .map(|_| ())
}

pub async fn get_custom_field_options(custom_field_id: i32, user_id: i32, db: &DatabaseConnection) -> PandaPartyResult<Vec<CustomCharacterFieldOption>> {
    custom_character_field_option::Entity::find()
        .filter(custom_character_field_option::Column::CustomCharacterFieldId.eq(custom_field_id))
        .filter(custom_character_field::Column::UserId.eq(user_id))
        .all(db)
        .await
        .map_err(|err| {
            log::error!("Failed to load custom field options: {err}");
            pandaparty_db_error!("character", "Failed to load custom field options")
        })
}

pub async fn create_custom_field_option(custom_field_id: i32, label: String, db: &DatabaseConnection) -> PandaPartyResult<CustomCharacterFieldOption> {
    custom_character_field_option::ActiveModel {
        id: NotSet,
        custom_character_field_id: Set(custom_field_id),
        label: Set(label),
    }
        .insert(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("character", "Failed to create custom field option")
        })
}

pub async fn update_custom_field_option(id: i32, custom_field_id: i32, option: String, db: &DatabaseConnection) -> PandaPartyErrorResult {
    custom_character_field_option::Entity::update_many()
        .filter(custom_character_field_option::Column::Id.eq(id))
        .filter(custom_character_field_option::Column::CustomCharacterFieldId.eq(custom_field_id))
        .col_expr(custom_character_field_option::Column::Label, Expr::value(option))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("character", "Failed to update custom field")
        })
        .map(|_| ())
}

pub async fn delete_custom_field_option(id: i32, custom_field_id: i32, db: &DatabaseConnection) -> PandaPartyErrorResult {
    custom_character_field_option::Entity::delete_many()
        .filter(custom_character_field_option::Column::Id.eq(id))
        .filter(custom_character_field_option::Column::CustomCharacterFieldId.eq(custom_field_id))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("character", "Failed to delete custom field")
        })
        .map(|_| ())
}

pub async fn custom_field_exists(id: i32, user_id: i32, db: &DatabaseConnection) -> bool {
    custom_character_field::Entity::find_by_id(id)
        .filter(custom_character_field::Column::UserId.eq(user_id))
        .select_only()
        .column(custom_character_field::Column::Id)
        .count(db)
        .await
        .map(|count| count > 0)
        .unwrap_or(false)
}

pub async fn custom_field_exists_by_label(label: String, user_id: i32, db: &DatabaseConnection) -> bool {
    custom_character_field::Entity::find()
        .filter(custom_character_field::Column::Label.eq(label))
        .filter(custom_character_field::Column::UserId.eq(user_id))
        .select_only()
        .column(custom_character_field::Column::Id)
        .count(db)
        .await
        .map(|count| count > 0)
        .unwrap_or(false)
}

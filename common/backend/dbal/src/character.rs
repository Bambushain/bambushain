use crate::error_tag;
use bamboo_common_core::entities::*;
use bamboo_common_core::entities::{
    character, custom_character_field, custom_character_field_option, custom_character_field_value,
};
use bamboo_common_core::error::*;
use character::CharacterRaceEnum;
use itertools::Itertools;
use sea_orm::prelude::*;
use sea_orm::sea_query::{Expr, IntoCondition};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    Condition, FromQueryResult, IntoActiveModel, JoinType, NotSet, QueryOrder, QuerySelect,
    SelectColumns, TransactionError, TransactionTrait,
};
use std::collections::BTreeSet;

fn map_character(
    character: CharacterWithFreeCompany,
    user_id: i32,
    custom_fields: Vec<FillCustomField>,
) -> Character {
    Character {
        id: character.id,
        race: CharacterRace::from(character.race),
        name: character.name.clone(),
        world: character.world.clone(),
        user_id,
        custom_fields: fill_custom_fields(user_id, custom_fields),
        free_company_id: character.free_company_id,
        free_company: character.free_company_name.map(|name| FreeCompany {
            id: character.free_company_id.unwrap(),
            name,
            user_id,
        }),
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default, FromQueryResult)]
struct CharacterWithFreeCompany {
    pub id: i32,
    pub user_id: i32,
    pub world: String,
    pub name: String,
    pub race: String,
    pub free_company_id: Option<i32>,
    pub free_company_name: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default, FromQueryResult)]
struct FillCustomField {
    pub character_id: i32,
    pub position: i32,
    pub option_label: String,
    pub field_label: String,
}

async fn get_custom_fields(
    additional_filter: impl IntoCondition,
    db: &DatabaseConnection,
) -> BambooResult<Vec<FillCustomField>> {
    custom_character_field_value::Entity::find()
        .select_only()
        .column_as(custom_character_field::Column::Label, "field_label")
        .column_as(custom_character_field::Column::Position, "position")
        .column_as(
            custom_character_field_value::Column::CharacterId,
            "character_id",
        )
        .column_as(custom_character_field_option::Column::Label, "option_label")
        .join_rev(
            JoinType::LeftJoin,
            custom_character_field::Entity::belongs_to(custom_character_field_value::Entity)
                .from(custom_character_field::Column::Id)
                .to(custom_character_field_value::Column::CustomCharacterFieldId)
                .into(),
        )
        .join_rev(
            JoinType::LeftJoin,
            custom_character_field_option::Entity::belongs_to(custom_character_field_value::Entity)
                .from(custom_character_field_option::Column::Id)
                .to(custom_character_field_value::Column::CustomCharacterFieldOptionId)
                .into(),
        )
        .join_rev(
            JoinType::LeftJoin,
            character::Entity::belongs_to(custom_character_field_value::Entity)
                .from(character::Column::Id)
                .to(custom_character_field_value::Column::CharacterId)
                .into(),
        )
        .filter(additional_filter)
        .order_by_asc(custom_character_field_value::Column::CharacterId)
        .order_by_asc(custom_character_field::Column::Position)
        .into_model::<FillCustomField>()
        .all(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to load custom fields"))
}

pub async fn get_characters(user_id: i32, db: &DatabaseConnection) -> BambooResult<Vec<Character>> {
    let custom_fields = get_custom_fields(character::Column::UserId.eq(user_id), db).await?;

    character::Entity::find()
        .select_only()
        .column(character::Column::Id)
        .column(character::Column::UserId)
        .column(character::Column::World)
        .column(character::Column::Name)
        .column(character::Column::Race)
        .column_as(free_company::Column::Id, "free_company_id")
        .column_as(free_company::Column::Name, "free_company_name")
        .join_rev(
            JoinType::LeftJoin,
            free_company::Entity::belongs_to(character::Entity)
                .from(free_company::Column::Id)
                .to(character::Column::FreeCompanyId)
                .into(),
        )
        .filter(character::Column::UserId.eq(user_id))
        .into_model::<CharacterWithFreeCompany>()
        .all(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to load characters"))
        .map(|characters| {
            characters
                .iter()
                .cloned()
                .map(|character| map_character(character, user_id, custom_fields.clone()))
                .collect_vec()
        })
}

pub async fn get_character(
    id: i32,
    user_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<Character> {
    let custom_fields = get_custom_fields(
        Condition::all()
            .add(character::Column::UserId.eq(user_id))
            .add(character::Column::Id.eq(id)),
        db,
    )
    .await?;

    character::Entity::find_by_id(id)
        .select_only()
        .column(character::Column::Id)
        .column(character::Column::UserId)
        .column(character::Column::World)
        .column(character::Column::Name)
        .column(character::Column::Race)
        .column_as(free_company::Column::Id, "free_company_id")
        .column_as(free_company::Column::Name, "free_company_name")
        .join_rev(
            JoinType::LeftJoin,
            free_company::Entity::belongs_to(character::Entity)
                .from(free_company::Column::Id)
                .to(character::Column::FreeCompanyId)
                .into(),
        )
        .filter(character::Column::UserId.eq(user_id))
        .order_by_asc(character::Column::Name)
        .into_model::<CharacterWithFreeCompany>()
        .one(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to load character"))?
        .map(|character| map_character(character, user_id, custom_fields))
        .ok_or(BambooError::not_found(error_tag!(), "Character not found"))
}

fn fill_custom_fields(character_id: i32, custom_fields: Vec<FillCustomField>) -> Vec<CustomField> {
    custom_fields
        .iter()
        .filter(|field| field.character_id == character_id)
        .chunk_by(|field| (field.field_label.clone(), field.position))
        .into_iter()
        .map(|((label, position), group)| CustomField {
            values: group
                .map(|item| item.option_label.clone())
                .collect::<BTreeSet<String>>(),
            label,
            position: position as usize,
        })
        .collect_vec()
}

async fn character_exists_by_id(
    id: i32,
    name: String,
    world: String,
    user_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<bool> {
    character::Entity::find()
        .filter(character::Column::Id.ne(id))
        .filter(character::Column::Name.eq(name))
        .filter(character::Column::World.eq(world))
        .filter(character::Column::UserId.eq(user_id))
        .count(db)
        .await
        .map(|count| count > 0)
        .map_err(|_| BambooError::database(error_tag!(), "Failed to load the characters"))
}

async fn character_exists_by_name(
    name: String,
    world: String,
    user_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<bool> {
    character::Entity::find()
        .filter(character::Column::Name.eq(name))
        .filter(character::Column::World.eq(world))
        .filter(character::Column::UserId.eq(user_id))
        .count(db)
        .await
        .map(|count| count > 0)
        .map_err(|_| BambooError::database(error_tag!(), "Failed to load the characters"))
}

pub async fn create_character(
    user_id: i32,
    character: Character,
    db: &DatabaseConnection,
) -> BambooResult<Character> {
    if character_exists_by_name(character.name.clone(), character.world.clone(), user_id, db)
        .await?
    {
        return Err(BambooError::exists_already(
            error_tag!(),
            "A character with that name already exists",
        ));
    }

    let mut model = character.clone().into_active_model();
    model.free_company_id = Set(character.free_company.map(|company| company.id));
    model.user_id = Set(user_id);
    model.id = NotSet;

    let model = model
        .insert(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to create character"))?;

    create_custom_field_values(user_id, model.id, character.custom_fields, db).await?;

    Ok(model)
}

pub async fn update_character(
    id: i32,
    user_id: i32,
    character: Character,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    if character_exists_by_id(
        id,
        character.name.clone(),
        character.world.clone(),
        user_id,
        db,
    )
    .await?
    {
        return Err(BambooError::exists_already(
            error_tag!(),
            "A character with that name already exists",
        ));
    }

    character::Entity::update_many()
        .filter(character::Column::Id.eq(id))
        .filter(character::Column::UserId.eq(user_id))
        .col_expr(character::Column::Name, Expr::value(character.name.clone()))
        .col_expr(
            character::Column::FreeCompanyId,
            Expr::value(character.free_company.map(|free_company| free_company.id)),
        )
        .col_expr(
            character::Column::World,
            Expr::value(character.world.clone()),
        )
        .col_expr(
            character::Column::Race,
            Expr::val(character.race).as_enum(CharacterRaceEnum),
        )
        .exec(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to update character"))?;

    create_custom_field_values(user_id, id, character.custom_fields, db).await
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default, FromQueryResult)]
struct CustomCharacterFieldCreated {
    pub character_id: i32,
    pub custom_character_field_id: i32,
    pub custom_character_field_option_id: i32,
}

async fn create_custom_field_values(
    user_id: i32,
    character_id: i32,
    custom_fields: Vec<CustomField>,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    if custom_fields.is_empty() {
        return Ok(());
    }

    db.transaction(move |tx| {
        Box::pin(async move {
            custom_character_field_value::Entity::delete_many()
                .filter(custom_character_field_value::Column::CharacterId.eq(character_id))
                .exec(tx)
                .await
                .map_err(|_| BambooError::database(error_tag!(), "Failed to set custom fields"))?;

            let field_labels = custom_fields
                .iter()
                .cloned()
                .map(|field| field.label)
                .collect_vec();
            let option_labels = custom_fields
                .iter()
                .map(|field| field.values.iter().cloned().collect_vec())
                .reduce(|acc, e| acc.iter().cloned().chain(e.iter().cloned()).collect_vec())
                .unwrap_or(Vec::new());

            let fields = custom_character_field::Entity::find()
                .select_only()
                .column_as(
                    custom_character_field::Column::Id,
                    custom_character_field_value::Column::CustomCharacterFieldId,
                )
                .column_as(
                    custom_character_field_option::Column::Id,
                    custom_character_field_value::Column::CustomCharacterFieldOptionId,
                )
                .select_column_as(
                    Expr::value(character_id),
                    custom_character_field_value::Column::CharacterId,
                )
                .left_join(custom_character_field_option::Entity)
                .filter(custom_character_field::Column::UserId.eq(user_id))
                .filter(custom_character_field::Column::Label.is_in(field_labels))
                .filter(custom_character_field_option::Column::Label.is_in(option_labels))
                .into_model::<CustomCharacterFieldValue>()
                .all(tx)
                .await
                .map_err(|_| BambooError::database(error_tag!(), "Failed to set custom fields"))?
                .iter()
                .cloned()
                .map(|field| field.into_active_model())
                .collect_vec();

            custom_character_field_value::Entity::insert_many(fields)
                .exec(tx)
                .await
                .map_err(|_| BambooError::database(error_tag!(), "Failed to set custom fields"))
                .map(|_| ())
        })
    })
    .await
    .map_err(|_: TransactionError<BambooError>| {
        BambooError::database(error_tag!(), "Failed to update grove mods")
    })
    .map(|_| ())
}

pub async fn delete_character(id: i32, user_id: i32, db: &DatabaseConnection) -> BambooErrorResult {
    character::Entity::delete_many()
        .filter(character::Column::Id.eq(id))
        .filter(character::Column::UserId.eq(user_id))
        .exec(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to delete character"))
        .map(|_| ())
}

use sea_orm::prelude::*;
use sea_orm::sea_query::Expr;
use sea_orm::ActiveValue::Set;
use sea_orm::{IntoSimpleExpr, NotSet, QueryOrder, QuerySelect};
use std::cmp::Ordering;

use bamboo_entities::prelude::*;
use bamboo_entities::{bamboo_db_error, custom_character_field, custom_character_field_option};

pub async fn get_custom_fields(
    user_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<Vec<CustomCharacterField>> {
    let fields = custom_character_field::Entity::find()
        .find_with_related(custom_character_field_option::Entity)
        .filter(custom_character_field::Column::UserId.eq(user_id))
        .order_by_asc(custom_character_field::Column::Position)
        .order_by_asc(custom_character_field::Column::Label)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            bamboo_db_error!("character", "Failed to load character custom fields")
        })?;

    Ok(fields
        .iter()
        .map(|(field, options)| CustomCharacterField {
            options: options.clone(),
            label: field.label.clone(),
            id: field.id,
            user_id: field.user_id,
            position: field.position,
        })
        .collect::<Vec<CustomCharacterField>>())
}

pub async fn get_custom_field(
    custom_field_id: i32,
    user_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<CustomCharacterField> {
    let fields = custom_character_field::Entity::find_by_id(custom_field_id)
        .find_with_related(custom_character_field_option::Entity)
        .filter(custom_character_field::Column::UserId.eq(user_id))
        .all(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            bamboo_db_error!("character", "Failed to load character custom fields")
        })?;

    let mut result = None;
    if let Some((field, options)) = fields.into_iter().next() {
        let mut f = field;
        f.options = options;
        result = Some(f);
    }

    if let Some(result) = result {
        Ok(result)
    } else {
        Err(bamboo_not_found_error!(
            "character",
            "Custom field not found"
        ))
    }
}

pub async fn create_custom_field(
    user_id: i32,
    custom_field: CustomField,
    db: &DatabaseConnection,
) -> BambooResult<CustomCharacterField> {
    custom_character_field::Entity::update_many()
        .filter(custom_character_field::Column::UserId.eq(user_id))
        .col_expr(
            custom_character_field::Column::Position,
            Expr::col(custom_character_field::Column::Position).add(1),
        )
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            bamboo_db_error!("character", "Failed to move custom field")
        })?;

    let result = custom_character_field::ActiveModel {
        id: NotSet,
        label: Set(custom_field.label),
        user_id: Set(user_id),
        position: Set(custom_field.position as i32),
    }
    .insert(db)
    .await
    .map_err(|err| {
        log::error!("{err}");
        bamboo_db_error!("character", "Failed to create custom field")
    })?;

    let models = custom_field
        .values
        .iter()
        .map(|value| custom_character_field_option::ActiveModel {
            id: NotSet,
            custom_character_field_id: Set(result.id),
            label: Set(value.clone()),
        })
        .collect::<Vec<custom_character_field_option::ActiveModel>>();

    if !models.is_empty() {
        custom_character_field_option::Entity::insert_many(models)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("{err}");
                bamboo_db_error!("character", "Failed to create custom field option")
            })?;
    }

    Ok(result)
}

pub async fn update_custom_field(
    id: i32,
    user_id: i32,
    custom_field: CustomField,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    custom_character_field::Entity::update_many()
        .filter(custom_character_field::Column::Id.eq(id))
        .filter(custom_character_field::Column::UserId.eq(user_id))
        .col_expr(
            custom_character_field::Column::Label,
            Expr::value(custom_field.label),
        )
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            bamboo_db_error!("character", "Failed to update custom field")
        })
        .map(|_| ())
}

pub async fn delete_custom_field(
    id: i32,
    user_id: i32,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    custom_character_field::Entity::delete_many()
        .filter(custom_character_field::Column::Id.eq(id))
        .filter(custom_character_field::Column::UserId.eq(user_id))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            bamboo_db_error!("character", "Failed to delete custom field")
        })
        .map(|_| ())
}

pub async fn get_custom_field_options(
    custom_field_id: i32,
    user_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<Vec<CustomCharacterFieldOption>> {
    custom_character_field_option::Entity::find()
        .filter(custom_character_field_option::Column::CustomCharacterFieldId.eq(custom_field_id))
        .filter(custom_character_field::Column::UserId.eq(user_id))
        .all(db)
        .await
        .map_err(|err| {
            log::error!("Failed to load custom field options: {err}");
            bamboo_db_error!("character", "Failed to load custom field options")
        })
}

pub async fn create_custom_field_option(
    custom_field_id: i32,
    label: String,
    db: &DatabaseConnection,
) -> BambooResult<CustomCharacterFieldOption> {
    custom_character_field_option::ActiveModel {
        id: NotSet,
        custom_character_field_id: Set(custom_field_id),
        label: Set(label),
    }
    .insert(db)
    .await
    .map_err(|err| {
        log::error!("{err}");
        bamboo_db_error!("character", "Failed to create custom field option")
    })
}

pub async fn update_custom_field_option(
    id: i32,
    custom_field_id: i32,
    option: String,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    custom_character_field_option::Entity::update_many()
        .filter(custom_character_field_option::Column::Id.eq(id))
        .filter(custom_character_field_option::Column::CustomCharacterFieldId.eq(custom_field_id))
        .col_expr(
            custom_character_field_option::Column::Label,
            Expr::value(option),
        )
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            bamboo_db_error!("character", "Failed to update custom field")
        })
        .map(|_| ())
}

pub async fn delete_custom_field_option(
    id: i32,
    custom_field_id: i32,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    custom_character_field_option::Entity::delete_many()
        .filter(custom_character_field_option::Column::Id.eq(id))
        .filter(custom_character_field_option::Column::CustomCharacterFieldId.eq(custom_field_id))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            bamboo_db_error!("character", "Failed to delete custom field")
        })
        .map(|_| ())
}

pub async fn custom_field_exists(user_id: i32, id: i32, db: &DatabaseConnection) -> bool {
    custom_character_field::Entity::find_by_id(id)
        .filter(custom_character_field::Column::UserId.eq(user_id))
        .select_only()
        .column(custom_character_field::Column::Id)
        .count(db)
        .await
        .map(|count| count > 0)
        .unwrap_or(false)
}

pub async fn custom_field_exists_by_label(
    label: String,
    user_id: i32,
    db: &DatabaseConnection,
) -> bool {
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

pub async fn move_custom_field(
    user_id: i32,
    field_id: i32,
    position: i32,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    let old_position = if let Some(old_position) = custom_character_field::Entity::find()
        .select_only()
        .column(custom_character_field::Column::Position)
        .filter(custom_character_field::Column::Id.eq(field_id))
        .into_tuple::<i32>()
        .one(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            bamboo_db_error!("character", "Failed to move custom field")
        })? {
        Ok(old_position)
    } else {
        Err(bamboo_db_error!("character", "Failed to move custom field"))
    }?;

    let (position_expr, filter_expr, new_position) = match old_position.cmp(&position) {
        Ordering::Less => (
            Expr::col(custom_character_field::Column::Position).sub(1),
            custom_character_field::Column::UserId
                .eq(user_id)
                .and(custom_character_field::Column::Position.lte(position)),
            position,
        ),
        Ordering::Greater => (
            Expr::col(custom_character_field::Column::Position).add(1),
            custom_character_field::Column::UserId
                .eq(user_id)
                .and(custom_character_field::Column::Position.lt(old_position))
                .and(custom_character_field::Column::Position.gte(position)),
            position,
        ),
        _ => (
            Expr::col(custom_character_field::Column::Position).into_simple_expr(),
            custom_character_field::Column::Id.eq(field_id),
            position,
        ),
    };

    custom_character_field::Entity::update_many()
        .filter(filter_expr)
        .col_expr(custom_character_field::Column::Position, position_expr)
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            bamboo_db_error!("character", "Failed to move custom field")
        })?;

    custom_character_field::Entity::update_many()
        .filter(custom_character_field::Column::UserId.eq(user_id))
        .filter(custom_character_field::Column::Id.eq(field_id))
        .col_expr(
            custom_character_field::Column::Position,
            Expr::value(new_position),
        )
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            bamboo_db_error!("character", "Failed to move custom field")
        })?;

    let fields = custom_character_field::Entity::find()
        .select_only()
        .column(custom_character_field::Column::Id)
        .filter(custom_character_field::Column::UserId.eq(user_id))
        .order_by_asc(custom_character_field::Column::Position)
        .into_tuple::<i32>()
        .all(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            bamboo_db_error!("character", "Failed to move custom field")
        })?;

    for (idx, id) in fields.iter().enumerate() {
        custom_character_field::Entity::update_many()
            .col_expr(
                custom_character_field::Column::Position,
                Expr::value(idx as i32),
            )
            .filter(custom_character_field::Column::Id.eq(*id))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("{err}");
                bamboo_db_error!("character", "Failed to move custom field")
            })?;
    }

    Ok(())
}

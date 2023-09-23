use sea_orm::prelude::*;
use sea_orm::sea_query::Expr;
use sea_orm::ActiveValue::Set;
use sea_orm::{NotSet, QueryOrder, QuerySelect};

use pandaparty_entities::prelude::*;
use pandaparty_entities::{
    custom_character_field, custom_character_field_option, pandaparty_db_error,
};

pub async fn get_custom_fields(
    user_id: i32,
    db: &DatabaseConnection,
) -> PandaPartyResult<Vec<CustomCharacterField>> {
    let fields = custom_character_field::Entity::find()
        .find_with_related(custom_character_field_option::Entity)
        .filter(custom_character_field::Column::UserId.eq(user_id))
        .order_by_asc(custom_character_field::Column::Position)
        .order_by_asc(custom_character_field::Column::Label)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("character", "Failed to load character custom fields")
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
) -> PandaPartyResult<CustomCharacterField> {
    let fields = custom_character_field::Entity::find_by_id(custom_field_id)
        .find_with_related(custom_character_field_option::Entity)
        .filter(custom_character_field::Column::UserId.eq(user_id))
        .all(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("character", "Failed to load character custom fields")
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
        Err(pandaparty_not_found_error!(
            "character",
            "Custom field not found"
        ))
    }
}

pub async fn create_custom_field(
    user_id: i32,
    custom_field: CustomField,
    db: &DatabaseConnection,
) -> PandaPartyResult<CustomCharacterField> {
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
        pandaparty_db_error!("character", "Failed to create custom field")
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
                pandaparty_db_error!("character", "Failed to create custom field option")
            })?;
    }

    Ok(result)
}

pub async fn update_custom_field(
    id: i32,
    user_id: i32,
    custom_field: CustomField,
    db: &DatabaseConnection,
) -> PandaPartyErrorResult {
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
            pandaparty_db_error!("character", "Failed to update custom field")
        })
        .map(|_| ())
}

pub async fn delete_custom_field(
    id: i32,
    user_id: i32,
    db: &DatabaseConnection,
) -> PandaPartyErrorResult {
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

pub async fn get_custom_field_options(
    custom_field_id: i32,
    user_id: i32,
    db: &DatabaseConnection,
) -> PandaPartyResult<Vec<CustomCharacterFieldOption>> {
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

pub async fn create_custom_field_option(
    custom_field_id: i32,
    label: String,
    db: &DatabaseConnection,
) -> PandaPartyResult<CustomCharacterFieldOption> {
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

pub async fn update_custom_field_option(
    id: i32,
    custom_field_id: i32,
    option: String,
    db: &DatabaseConnection,
) -> PandaPartyErrorResult {
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
            pandaparty_db_error!("character", "Failed to update custom field")
        })
        .map(|_| ())
}

pub async fn delete_custom_field_option(
    id: i32,
    custom_field_id: i32,
    db: &DatabaseConnection,
) -> PandaPartyErrorResult {
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
) -> PandaPartyErrorResult {
    let current_field = custom_character_field::Entity::find_by_id(field_id)
        .filter(custom_character_field::Column::UserId.eq(user_id))
        .one(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            pandaparty_db_error!("character", "Failed to move custom field")
        })?;

    if let Some(field) = current_field {
        custom_character_field::Entity::update_many()
            .filter(custom_character_field::Column::Position.eq(position))
            .col_expr(
                custom_character_field::Column::Position,
                Expr::value(field.position),
            )
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("{err}");
                pandaparty_db_error!("character", "Failed to move custom field")
            })?;
        custom_character_field::Entity::update_many()
            .filter(custom_character_field::Column::Id.eq(field.id))
            .col_expr(
                custom_character_field::Column::Position,
                Expr::value(position),
            )
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("{err}");
                pandaparty_db_error!("character", "Failed to move custom field")
            })?;
    } else {
        return Err(pandaparty_not_found_error!("character", "Field not found"));
    };
    //
    // let mut prev_position = position;
    // for old_field in old_fields {
    //     prev_position += 1;
    //     custom_character_field::Entity::update_many()
    //         .col_expr(custom_character_field::Column::Position, Expr::value(prev_position))
    //         .filter(custom_character_field::Column::Id.eq(old_field))
    //         .exec(db)
    //         .await
    //         .map_err(|err| {
    //             log::error!("{err}");
    //             pandaparty_db_error!("character", "Failed to move custom field")
    //         })?;
    // }
    //
    // custom_character_field::Entity::update_many()
    //     .filter(custom_character_field::Column::UserId.eq(user_id))
    //     .filter(custom_character_field::Column::Id.eq(field_id))
    //     .col_expr(custom_character_field::Column::Position, Expr::value(position))
    //     .exec(db)
    //     .await
    //     .map_err(|err| {
    //         log::error!("{err}");
    //         pandaparty_db_error!("character", "Failed to move custom field")
    //     })?;
    //
    // let fields = custom_character_field::Entity::find()
    //     .select_only()
    //     .column(custom_character_field::Column::Id)
    //     .filter(custom_character_field::Column::UserId.eq(user_id))
    //     .order_by_asc(custom_character_field::Column::Position)
    //     .into_tuple::<i32>()
    //     .all(db)
    //     .await
    //     .map_err(|err| {
    //         log::error!("{err}");
    //         pandaparty_db_error!("character", "Failed to move custom field")
    //     })?;
    //
    // for (idx, id) in fields.iter().enumerate() {
    //     custom_character_field::Entity::update_many()
    //         .col_expr(custom_character_field::Column::Position, Expr::value(idx as i32))
    //         .filter(custom_character_field::Column::Id.eq(*id))
    //         .exec(db)
    //         .await
    //         .map_err(|err| {
    //             log::error!("{err}");
    //             pandaparty_db_error!("character", "Failed to move custom field")
    //         })?;
    // }

    Ok(())
}

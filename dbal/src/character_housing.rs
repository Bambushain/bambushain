use sea_orm::prelude::*;
use sea_orm::sea_query::Expr;
use sea_orm::ActiveValue::Set;
use sea_orm::{IntoActiveModel, NotSet, QueryOrder};

use bamboo_entities::prelude::*;
use bamboo_entities::{character, character_housing};
use bamboo_error::*;

pub async fn get_character_housings(
    user_id: i32,
    character_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<Vec<CharacterHousing>> {
    character_housing::Entity::find()
        .filter(character_housing::Column::CharacterId.eq(character_id))
        .filter(character::Column::UserId.eq(user_id))
        .inner_join(character::Entity)
        .order_by_asc(character_housing::Column::District)
        .order_by_asc(character_housing::Column::Ward)
        .order_by_asc(character_housing::Column::Plot)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("character_housing", "Failed to load character housings")
        })
}

pub async fn get_character_housing(
    id: i32,
    user_id: i32,
    character_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<CharacterHousing> {
    character_housing::Entity::find_by_id(id)
        .filter(character_housing::Column::CharacterId.eq(character_id))
        .filter(character::Column::UserId.eq(user_id))
        .inner_join(character::Entity)
        .one(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("character_housing", "Failed to load character housing")
        })
        .map(|res| {
            if let Some(res) = res {
                Ok(res)
            } else {
                Err(BambooError::not_found(
                    "character_housing",
                    "The character housing was not found",
                ))
            }
        })?
}

async fn character_housing_exists_by_id(
    id: i32,
    user_id: i32,
    character_id: i32,
    district: HousingDistrict,
    ward: u8,
    plot: u8,
    db: &DatabaseConnection,
) -> BambooResult<bool> {
    character_housing::Entity::find_by_id(id)
        .filter(character_housing::Column::CharacterId.eq(character_id))
        .filter(character_housing::Column::District.eq(district))
        .filter(character_housing::Column::Ward.eq(ward))
        .filter(character_housing::Column::Plot.eq(plot))
        .filter(character::Column::UserId.eq(user_id))
        .inner_join(character::Entity)
        .count(db)
        .await
        .map(|count| count > 0)
        .map_err(|err| {
            log::error!("Failed to load character housing {err}");
            BambooError::database("character_housing", "Failed to load the character housings")
        })
}

async fn character_housing_exists_by_fields(
    user_id: i32,
    character_id: i32,
    district: HousingDistrict,
    ward: u8,
    plot: u8,
    db: &DatabaseConnection,
) -> BambooResult<bool> {
    character_housing::Entity::find()
        .filter(character_housing::Column::CharacterId.eq(character_id))
        .filter(character_housing::Column::District.eq(district))
        .filter(character_housing::Column::Ward.eq(ward))
        .filter(character_housing::Column::Plot.eq(plot))
        .filter(character::Column::UserId.eq(user_id))
        .inner_join(character::Entity)
        .count(db)
        .await
        .map(|count| count > 0)
        .map_err(|err| {
            log::error!("Failed to load character housing {err}");
            BambooError::database("character_housing", "Failed to load the character housings")
        })
}

pub async fn create_character_housing(
    user_id: i32,
    character_id: i32,
    housing: CharacterHousing,
    db: &DatabaseConnection,
) -> BambooResult<CharacterHousing> {
    if character_housing_exists_by_fields(
        user_id,
        character_id,
        housing.district,
        housing.ward,
        housing.plot,
        db,
    )
    .await?
    {
        return Err(BambooError::exists_already(
            "character_housing",
            "A character housing with that job exists already",
        ));
    }

    let mut model = housing.into_active_model();
    model.character_id = Set(character_id);
    model.id = NotSet;

    model.insert(db).await.map_err(|err| {
        log::error!("{err}");
        BambooError::database("character_housing", "Failed to create character_housing")
    })
}

pub async fn update_character_housing(
    id: i32,
    user_id: i32,
    character_id: i32,
    housing: CharacterHousing,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    if character_housing_exists_by_id(
        id,
        user_id,
        character_id,
        housing.district,
        housing.ward,
        housing.plot,
        db,
    )
    .await?
    {
        return Err(BambooError::exists_already(
            "character_housing",
            "A character housing with that job exists already",
        ));
    }

    character_housing::Entity::update_many()
        .filter(character_housing::Column::Id.eq(id))
        .filter(character_housing::Column::CharacterId.eq(character_id))
        .col_expr(character_housing::Column::District, Expr::value(housing.district))
        .col_expr(character_housing::Column::Ward, Expr::value(housing.ward))
        .col_expr(character_housing::Column::Plot, Expr::value(housing.plot))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("character_housing", "Failed to update character housing")
        })
        .map(|_| ())
}

pub async fn delete_character_housing(
    id: i32,
    user_id: i32,
    character_id: i32,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    character_housing::Entity::delete_many()
        .filter(character_housing::Column::Id.eq(id))
        .filter(character_housing::Column::CharacterId.eq(character_id))
        .filter(character::Column::UserId.eq(user_id))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("character_housing", "Failed to delete character housing")
        })
        .map(|_| ())
}

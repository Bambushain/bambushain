use sea_orm::prelude::*;
use sea_orm::{NotSet, QueryOrder, Set};

use bamboo_entities::prelude::*;
use bamboo_error::*;

pub async fn get_free_companies(
    user_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<Vec<FreeCompany>> {
    free_company::Entity::find()
        .filter(free_company::Column::UserId.eq(user_id))
        .order_by_asc(free_company::Column::Name)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("Failed to load free companies: {err}");
            BambooError::not_found("free_company", "Free companies not found")
        })
}

pub async fn get_free_company(
    free_company_id: Option<i32>,
    user_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<Option<FreeCompany>> {
    if let Some(id) = free_company_id {
        free_company::Entity::find_by_id(id)
            .filter(free_company::Column::UserId.eq(user_id))
            .one(db)
            .await
            .map_err(|err| {
                log::error!("Failed to load free company: {err}");
                BambooError::not_found("free_company", "Free company not found")
            })
    } else {
        Ok(None)
    }
}

pub async fn create_free_company(
    user_id: i32,
    name: String,
    db: &DatabaseConnection,
) -> BambooResult<FreeCompany> {
    if free_company_exists_by_name(name.clone(), user_id, db).await? {
        return Err(BambooError::exists_already(
            "free company",
            "A free company with that name exists",
        ));
    }

    let mut active_model = free_company::ActiveModel::new();
    active_model.user_id = Set(user_id);
    active_model.name = Set(name);
    active_model.id = NotSet;

    active_model.insert(db).await.map_err(|err| {
        log::error!("Failed to create free company: {err}");
        BambooError::not_found("free_company", "Could not create free company")
    })
}

pub async fn update_free_company(
    id: i32,
    user_id: i32,
    name: String,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    if free_company_exists_by_id(name.clone(), user_id, id, db).await? {
        return Err(BambooError::exists_already(
            "free company",
            "A free company with that name exists",
        ));
    }

    free_company::Entity::update_many()
        .filter(free_company::Column::UserId.eq(user_id))
        .filter(free_company::Column::Id.eq(id))
        .col_expr(free_company::Column::Name, Expr::value(name))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("Failed to update free company: {err}");
            BambooError::not_found("free_company", "Could not update free company")
        })
        .map(|_| ())
}

pub async fn delete_free_company(
    id: i32,
    user_id: i32,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    free_company::Entity::delete_by_id(id)
        .filter(free_company::Column::UserId.eq(user_id))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("Failed to delete free company: {err}");
            BambooError::not_found("free_company", "Could not delete free company")
        })
        .map(|_| ())
}

async fn free_company_exists_by_id(
    name: String,
    user_id: i32,
    id: i32,
    db: &DatabaseConnection,
) -> BambooResult<bool> {
    free_company::Entity::find()
        .filter(free_company::Column::Id.ne(id))
        .filter(free_company::Column::Name.eq(name))
        .filter(free_company::Column::UserId.eq(user_id))
        .count(db)
        .await
        .map(|count| count > 0)
        .map_err(|err| {
            log::error!("Failed to load free companies {err}");
            BambooError::database("free company", "Failed to load free companies")
        })
}

async fn free_company_exists_by_name(
    name: String,
    user_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<bool> {
    free_company::Entity::find()
        .filter(free_company::Column::Name.eq(name))
        .filter(free_company::Column::UserId.eq(user_id))
        .count(db)
        .await
        .map(|count| count > 0)
        .map_err(|err| {
            log::error!("Failed to load free companies {err}");
            BambooError::database("free company", "Failed to load free companies")
        })
}

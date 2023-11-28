use bamboo_entities::prelude::*;
use sea_orm::prelude::*;
use sea_orm::{NotSet, QueryOrder, QuerySelect, Set};

pub async fn get_free_companies(
    user_id: i32,
    db: &DatabaseConnection,
) -> PandaPartyResult<Vec<FreeCompany>> {
    bamboo_entities::free_company::Entity::find()
        .filter(bamboo_entities::free_company::Column::UserId.eq(user_id))
        .order_by_asc(bamboo_entities::free_company::Column::Name)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("Failed to load free companies: {err}");
            bamboo_not_found_error!("free_company", "Free companies not found")
        })
}

pub async fn get_free_company(
    free_company_id: Option<i32>,
    user_id: i32,
    db: &DatabaseConnection,
) -> PandaPartyResult<Option<FreeCompany>> {
    if let Some(id) = free_company_id {
        bamboo_entities::free_company::Entity::find_by_id(id)
            .filter(bamboo_entities::free_company::Column::UserId.eq(user_id))
            .one(db)
            .await
            .map_err(|err| {
                log::error!("Failed to load free company: {err}");
                bamboo_not_found_error!("free_company", "Free company not found")
            })
    } else {
        Ok(None)
    }
}

pub async fn create_free_company(
    user_id: i32,
    name: String,
    db: &DatabaseConnection,
) -> PandaPartyResult<FreeCompany> {
    let mut active_model = bamboo_entities::free_company::ActiveModel::new();
    active_model.user_id = Set(user_id);
    active_model.name = Set(name);
    active_model.id = NotSet;

    active_model.insert(db).await.map_err(|err| {
        log::error!("Failed to create free company: {err}");
        bamboo_not_found_error!("free_company", "Could not create free company")
    })
}

pub async fn update_free_company(
    id: i32,
    user_id: i32,
    name: String,
    db: &DatabaseConnection,
) -> PandaPartyErrorResult {
    bamboo_entities::free_company::Entity::update_many()
        .filter(bamboo_entities::free_company::Column::UserId.eq(user_id))
        .filter(bamboo_entities::free_company::Column::Id.eq(id))
        .col_expr(
            bamboo_entities::free_company::Column::Name,
            Expr::value(name),
        )
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("Failed to update free company: {err}");
            bamboo_not_found_error!("free_company", "Could not update free company")
        })
        .map(|_| ())
}

pub async fn delete_free_company(
    id: i32,
    user_id: i32,
    db: &DatabaseConnection,
) -> PandaPartyErrorResult {
    bamboo_entities::free_company::Entity::delete_by_id(id)
        .filter(bamboo_entities::free_company::Column::UserId.eq(user_id))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("Failed to delete free company: {err}");
            bamboo_not_found_error!("free_company", "Could not delete free company")
        })
        .map(|_| ())
}

pub async fn free_company_exists(user_id: i32, id: i32, db: &DatabaseConnection) -> bool {
    bamboo_entities::free_company::Entity::find_by_id(id)
        .filter(bamboo_entities::free_company::Column::UserId.eq(user_id))
        .select_only()
        .column(bamboo_entities::free_company::Column::Id)
        .count(db)
        .await
        .map(|count| count > 0)
        .unwrap_or(false)
}

pub async fn free_company_exists_by_name(
    name: String,
    user_id: i32,
    db: &DatabaseConnection,
) -> bool {
    bamboo_entities::free_company::Entity::find()
        .filter(bamboo_entities::free_company::Column::Name.eq(name))
        .filter(bamboo_entities::free_company::Column::UserId.eq(user_id))
        .select_only()
        .column(bamboo_entities::free_company::Column::Id)
        .count(db)
        .await
        .map(|count| count > 0)
        .unwrap_or(false)
}

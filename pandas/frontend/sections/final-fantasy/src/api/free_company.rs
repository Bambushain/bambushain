use bamboo_common::core::entities::FreeCompany;
use bamboo_pandas_frontend_base::api::{delete, get, post, put_no_content, BambooApiResult};

pub async fn get_free_companies() -> BambooApiResult<Vec<FreeCompany>> {
    log::debug!("Get free companies");
    get("/api/final-fantasy/free-company").await
}

pub async fn create_free_company(free_company: FreeCompany) -> BambooApiResult<FreeCompany> {
    log::debug!("Create free company {}", free_company.name);
    post("/api/final-fantasy/free-company", &free_company).await
}

pub async fn update_free_company(id: i32, free_company: FreeCompany) -> BambooApiResult<()> {
    log::debug!("Update free company {id}");
    put_no_content(
        format!("/api/final-fantasy/free-company/{id}"),
        &free_company,
    )
    .await
}

pub async fn delete_free_company(id: i32) -> BambooApiResult<()> {
    log::debug!("Delete free company {id}");
    delete(format!("/api/final-fantasy/free-company/{id}")).await
}

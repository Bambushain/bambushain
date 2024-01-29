use bamboo_common::core::entities::grove::CreateGroveRequest;
use bamboo_common::core::entities::Grove;
use bamboo_common::frontend::api::{delete, get, post, put_no_body_no_content, BambooApiResult};

pub async fn get_groves() -> BambooApiResult<Vec<Grove>> {
    log::debug!("Get all groves");
    get("/api/grove").await
}

pub async fn create_grove(
    grove_name: String,
    mod_name: String,
    mod_email: String,
) -> BambooApiResult<Grove> {
    log::debug!("Create new grove {grove_name}");
    post(
        "/api/grove",
        &CreateGroveRequest::new(grove_name, mod_name, mod_email),
    )
    .await
}

pub async fn delete_grove(id: i32) -> BambooApiResult<()> {
    log::debug!("Delete grove {id}");
    delete(format!("/api/grove/{id}")).await
}

pub async fn suspend_grove(id: i32) -> BambooApiResult<()> {
    log::debug!("Suspend grove {id}");
    delete(format!("/api/grove/{id}/suspension")).await
}

pub async fn resume_grove(id: i32) -> BambooApiResult<()> {
    log::debug!("Resume grove {id}");
    put_no_body_no_content(format!("/api/grove/{id}/suspension")).await
}

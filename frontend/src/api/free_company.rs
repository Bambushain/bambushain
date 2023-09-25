use crate::api::{delete, get, post, put_no_content, ApiError, PandapartyApiResult};
use async_trait::async_trait;
use bounce::query::{Query, QueryResult};
use bounce::BounceStates;
use pandaparty_entities::prelude::FreeCompany;
use std::rc::Rc;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct FreeCompanies {
    pub free_companies: Vec<FreeCompany>,
}

impl From<Vec<FreeCompany>> for FreeCompanies {
    fn from(value: Vec<FreeCompany>) -> Self {
        Self {
            free_companies: value,
        }
    }
}

#[async_trait(? Send)]
impl Query for FreeCompanies {
    type Input = ();
    type Error = ApiError;

    async fn query(_states: &BounceStates, _input: Rc<Self::Input>) -> QueryResult<Self> {
        get_free_companies().await.map(|data| Rc::new(data.into()))
    }
}

pub async fn get_free_companies() -> PandapartyApiResult<Vec<FreeCompany>> {
    log::debug!("Get free companies");
    get("/api/final-fantasy/free-company").await
}

pub async fn create_free_company(free_company: FreeCompany) -> PandapartyApiResult<FreeCompany> {
    log::debug!("Create free company {}", free_company.name);
    post("/api/final-fantasy/free-company", &free_company).await
}

pub async fn update_free_company(id: i32, free_company: FreeCompany) -> PandapartyApiResult<()> {
    log::debug!("Update free company {id}");
    put_no_content(
        format!("/api/final-fantasy/free-company/{id}"),
        &free_company,
    )
    .await
}

pub async fn delete_free_company(id: i32) -> PandapartyApiResult<()> {
    log::debug!("Delete free company {id}");
    delete(format!("/api/final-fantasy/free-company/{id}")).await
}

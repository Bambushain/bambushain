use std::rc::Rc;
use async_trait::async_trait;
use bounce::prelude::*;
use bounce::query::{Mutation, MutationResult, Query, QueryResult};
use serde::{Deserialize, Serialize};
use crate::api::{ErrorCode, get};
use crate::api::authentication::login;
use crate::storage::{delete_token, set_token};

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct Profile {
    pub user: sheef_entities::User,
}

impl From<sheef_entities::User> for Profile {
    fn from(value: sheef_entities::User) -> Self {
        Self {
            user: value
        }
    }
}

async fn get_my_profile() -> Result<sheef_entities::User, ErrorCode> {
    log::debug!("Get my profile");
    get::<sheef_entities::User>("/api/my/profile").await
}

#[async_trait(? Send)]
impl Query for Profile {
    type Input = ();
    type Error = ErrorCode;

    async fn query(_states: &BounceStates, _input: Rc<Self::Input>) -> QueryResult<Self> {
        match get_my_profile().await {
            Ok(user) => Ok(Rc::new(user.into())),
            Err(err) => {
                delete_token();
                Err(err)
            }
        }
    }
}

#[async_trait(? Send)]
impl Mutation for Profile {
    type Input = sheef_entities::Login;
    type Error = ErrorCode;

    async fn run(_states: &BounceStates, input: Rc<Self::Input>) -> MutationResult<Self> {
        match login(input).await {
            Ok(result) => {
                set_token(result.token);
                Ok(Rc::new(result.user.into()))
            }
            Err(err) => Err(err)
        }
    }
}

use std::rc::Rc;
use async_trait::async_trait;
use bounce::prelude::*;
use bounce::query::{Mutation, MutationResult, Query, QueryResult};
use serde::{Deserialize, Serialize};
use crate::api::{ErrorCode, get, login};
use crate::storage::{delete_token, set_token};

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct Profile(sheef_entities::User);

impl From<sheef_entities::User> for Profile {
    fn from(value: sheef_entities::User) -> Self {
        Profile(value)
    }
}

async fn get_my_profile() -> Result<Profile, ErrorCode> {
    log::debug!("Get my profile");
    get::<Profile>("/api/my/profile").await
}

#[async_trait(? Send)]
impl Query for Profile {
    type Input = ();
    type Error = ErrorCode;

    async fn query(_states: &BounceStates, _input: Rc<Self::Input>) -> QueryResult<Self> {
        match get_my_profile().await {
            Ok(user) => Ok(user.into()),
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
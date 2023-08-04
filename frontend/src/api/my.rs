use std::rc::Rc;

use async_trait::async_trait;
use bounce::prelude::*;
use bounce::query::{Mutation, MutationResult, Query, QueryResult};
use serde::{Deserialize, Serialize};

use pandaparty_entities::prelude::*;

use crate::api::{ApiError, delete, get, put, put_no_body, SheefApiResult};
use crate::api::authentication::login;
use crate::storage::{delete_token, set_token};

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct Profile {
    pub user: WebUser,
}

impl From<WebUser> for Profile {
    fn from(value: WebUser) -> Self {
        Self {
            user: value
        }
    }
}

async fn get_my_profile() -> SheefApiResult<WebUser> {
    log::debug!("Get my profile");
    get::<WebUser>("/api/my/profile").await
}

#[async_trait(? Send)]
impl Query for Profile {
    type Input = ();
    type Error = ApiError;

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
    type Input = Login;
    type Error = ApiError;

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

pub async fn change_my_password(old_password: String, new_password: String) -> SheefApiResult<()> {
    log::debug!("Change my password");
    put("/api/my/password", &ChangeMyPassword { old_password, new_password }).await
}

pub async fn update_my_profile(profile: UpdateProfile) -> SheefApiResult<()> {
    log::debug!("Update profile to the following data {:?}", profile);
    put("/api/my/profile", &profile).await
}

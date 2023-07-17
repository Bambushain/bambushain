use std::rc::Rc;
use async_trait::async_trait;
use bounce::prelude::*;
use bounce::query::{Mutation, MutationResult, Query, QueryResult};
use serde::{Deserialize, Serialize};
use sheef_entities::authentication::ChangeMyPassword;
use sheef_entities::UpdateProfile;
use crate::api::{delete, ErrorCode, get, put, put_no_body};
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

pub async fn activate_kill_for_me(name: String) -> ErrorCode {
    log::debug!("Activate kill {name} for me");
    put_no_body(format!("/api/my/kill/{name}")).await
}

pub async fn deactivate_kill_for_me(name: String) -> ErrorCode {
    log::debug!("Deactivate kill {name} for me");
    delete(format!("/api/my/kill/{name}")).await
}

pub async fn activate_mount_for_me(name: String) -> ErrorCode {
    log::debug!("Activate mount {name} for me");
    put_no_body(format!("/api/my/mount/{name}")).await
}

pub async fn deactivate_mount_for_me(name: String) -> ErrorCode {
    log::debug!("Deactivate mount {name} for me");
    delete(format!("/api/my/mount/{name}")).await
}

pub async fn activate_savage_mount_for_me(name: String) -> ErrorCode {
    log::debug!("Activate savage mount {name} for me");
    put_no_body(format!("/api/my/savage-mount/{name}")).await
}

pub async fn deactivate_savage_mount_for_me(name: String) -> ErrorCode {
    log::debug!("Deactivate savage mount {name} for me");
    delete(format!("/api/my/savage-mount/{name}")).await
}

pub async fn change_my_password(old_password: String, new_password: String) -> ErrorCode {
    log::debug!("Change my password");
    put("/api/my/password", &ChangeMyPassword { old_password, new_password }).await
}

pub async fn update_my_profile(profile: UpdateProfile) -> ErrorCode {
    log::debug!("Update profile to the following data {:?}", profile);
    put("/api/my/profile", &profile).await
}
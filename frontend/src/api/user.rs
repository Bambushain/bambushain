use std::rc::Rc;

use async_trait::async_trait;
use bounce::query::{Query, QueryResult};
use bounce::BounceStates;

use bamboo_entities::prelude::*;

use crate::api::{
    delete, get, post, put_no_body_no_content, put_no_content, ApiError, BambooApiResult,
};

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct Users {
    pub users: Vec<WebUser>,
}

impl From<Vec<WebUser>> for Users {
    fn from(value: Vec<WebUser>) -> Self {
        Self { users: value }
    }
}

pub(crate) async fn get_users() -> BambooApiResult<Vec<WebUser>> {
    log::debug!("Get users");
    get("/api/user").await
}

#[async_trait(? Send)]
impl Query for Users {
    type Input = ();
    type Error = ApiError;

    async fn query(_states: &BounceStates, _input: Rc<Self::Input>) -> QueryResult<Self> {
        get_users().await.map(|res| Rc::new(res.into()))
    }
}

pub async fn create_user(user: User) -> BambooApiResult<WebUser> {
    log::debug!("Create user {}", user.email);
    post("/api/user", &user).await
}

pub async fn make_user_mod(id: i32) -> BambooApiResult<()> {
    log::debug!("Make user {id} mod");
    put_no_body_no_content(format!("/api/user/{id}/mod")).await
}

pub async fn remove_user_mod(id: i32) -> BambooApiResult<()> {
    log::debug!("Remove user {id} mod");
    delete(format!("/api/user/{id}/mod")).await
}

pub async fn delete_user(id: i32) -> BambooApiResult<()> {
    log::debug!("Remove user {id} main");
    delete(format!("/api/user/{id}")).await
}

pub async fn change_user_password(id: i32, new_password: String) -> BambooApiResult<()> {
    log::debug!("Change user {id} password");
    put_no_content(
        format!("/api/user/{id}/password"),
        &ChangePassword { new_password },
    )
    .await
}

pub async fn update_profile(id: i32, profile: UpdateProfile) -> BambooApiResult<()> {
    log::debug!(
        "Update profile of user {id} to the following data {:?}",
        profile
    );
    put_no_content(format!("/api/user/{id}/profile"), &profile).await
}

use std::rc::Rc;

use async_trait::async_trait;
use bounce::BounceStates;
use bounce::query::{Query, QueryResult};

use pandaparty_entities::prelude::*;

use crate::api::{ApiError, delete, get, post, put, put_no_body, SheefApiResult};

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct Users {
    pub users: Vec<WebUser>,
}

impl From<Vec<WebUser>> for Users {
    fn from(value: Vec<WebUser>) -> Self {
        Self {
            users: value
        }
    }
}

pub(crate) async fn get_users() -> SheefApiResult<Vec<WebUser>> {
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

pub async fn create_user(user: User) -> SheefApiResult<WebUser> {
    log::debug!("Create user {}", user.username);
    post("/api/user", &user).await
}

pub async fn make_user_mod(id: i32) -> SheefApiResult<()> {
    log::debug!("Make user {id} mod");
    put_no_body(format!("/api/user/{id}/mod")).await
}

pub async fn remove_user_mod(id: i32) -> SheefApiResult<()> {
    log::debug!("Remove user {id} mod");
    delete(format!("/api/user/{id}/mod")).await
}

pub async fn delete_user(id: i32) -> SheefApiResult<()> {
    log::debug!("Remove user {id} main");
    delete(format!("/api/user/{id}")).await
}

pub async fn change_user_password(id: i32, new_password: String) -> SheefApiResult<()> {
    log::debug!("Change user {id} password");
    put(format!("/api/user/{id}/password"), &ChangePassword { new_password }).await
}

pub async fn update_profile(id: i32, profile: UpdateProfile) -> SheefApiResult<()> {
    log::debug!("Update profile of user {id} to the following data {:?}", profile);
    put(format!("/api/user/{id}/profile"), &profile).await
}

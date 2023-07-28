use std::rc::Rc;

use async_trait::async_trait;
use bounce::BounceStates;
use bounce::query::{Query, QueryResult};

use sheef_entities::prelude::*;

use crate::api::{ApiError, delete, get, post, put, put_no_body, SheefApiResult};

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct Crew {
    pub users: Vec<WebUser>,
}

impl From<Vec<WebUser>> for Crew {
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
impl Query for Crew {
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

pub async fn make_user_mod(user: WebUser) -> SheefApiResult<()> {
    log::debug!("Make user {} mod", user.username);
    put_no_body(format!("/api/user/{}/mod", user.username)).await
}

pub async fn remove_user_mod(user: WebUser) -> SheefApiResult<()> {
    log::debug!("Remove user {} mod", user.username);
    delete(format!("/api/user/{}/mod", user.username)).await
}

pub async fn delete_user(user: WebUser) -> SheefApiResult<()> {
    log::debug!("Remove user {} main", user.username);
    delete(format!("/api/user/{}", user.username)).await
}

pub async fn change_user_password(user: WebUser, new_password: String) -> SheefApiResult<()> {
    log::debug!("Change user {} password", user.username);
    put(format!("/api/user/{}/password", user.username), &ChangePassword { new_password }).await
}

pub async fn update_profile(profile: UpdateProfile, username: String) -> SheefApiResult<()> {
    log::debug!("Update profile of user {username} to the following data {:?}", profile);
    put(format!("/api/user/{username}/profile"), &profile).await
}

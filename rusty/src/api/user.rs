use std::rc::Rc;
use async_trait::async_trait;
use bounce::BounceStates;
use bounce::query::{Query, QueryResult};
use sheef_entities::authentication::ChangePassword;
use crate::api::{delete, ErrorCode, get, post, put_no_body, put_no_response};

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct Crew {
    pub users: Vec<sheef_entities::User>,
}

impl From<Vec<sheef_entities::User>> for Crew {
    fn from(value: Vec<sheef_entities::User>) -> Self {
        Self {
            users: value
        }
    }
}

async fn get_users() -> Result<Vec<sheef_entities::User>, ErrorCode> {
    log::debug!("Get users");
    get("/api/user").await
}

#[async_trait(? Send)]
impl Query for Crew {
    type Input = ();
    type Error = ErrorCode;

    async fn query(_states: &BounceStates, _input: Rc<Self::Input>) -> QueryResult<Self> {
        get_users().await.map(|res| Rc::new(res.into()))
    }
}

pub async fn create_user(user: sheef_entities::user::User) -> Result<sheef_entities::User, ErrorCode> {
    log::debug!("Create user {}", user.username);
    post("/api/user", user.into()).await
}

pub async fn make_user_mod(user: sheef_entities::User) -> ErrorCode {
    log::debug!("Make user {} mod", user.username);
    put_no_body(format!("/api/user/{}/mod", user.username)).await
}

pub async fn remove_user_mod(user: sheef_entities::User) -> ErrorCode {
    log::debug!("Remove user {} mod", user.username);
    delete(format!("/api/user/{}/mod", user.username)).await
}

pub async fn make_user_main(user: sheef_entities::User) -> ErrorCode {
    log::debug!("Make user {} main", user.username);
    put_no_body(format!("/api/user/{}/main", user.username)).await
}

pub async fn remove_user_main(user: sheef_entities::User) -> ErrorCode {
    log::debug!("Remove user {} main", user.username);
    delete(format!("/api/user/{}/main", user.username)).await
}

pub async fn delete_user(user: sheef_entities::User) -> ErrorCode {
    log::debug!("Remove user {} main", user.username);
    delete(format!("/api/user/{}", user.username)).await
}

pub async fn change_user_password(user: sheef_entities::User, new_password: String) -> ErrorCode {
    log::debug!("Change user {} password", user.username);
    put_no_response(format!("/api/user/{}/password", user.username), ChangePassword { new_password }.into()).await
}
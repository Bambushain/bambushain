use std::rc::Rc;

use async_trait::async_trait;
use bounce::prelude::*;
use bounce::query::{Query, QueryResult};
use serde::{Deserialize, Serialize};

use pandaparty_entities::prelude::*;

use crate::api;
use crate::storage;

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

async fn get_my_profile() -> api::PandapartyApiResult<WebUser> {
    log::debug!("Get my profile");
    api::get::<WebUser>("/api/my/profile").await
}

#[async_trait(? Send)]
impl Query for Profile {
    type Input = ();
    type Error = api::ApiError;

    async fn query(_states: &BounceStates, _input: Rc<Self::Input>) -> QueryResult<Self> {
        match get_my_profile().await {
            Ok(user) => Ok(Rc::new(user.into())),
            Err(err) => {
                storage::delete_token();
                Err(err)
            }
        }
    }
}

pub async fn change_my_password(old_password: String, new_password: String) -> api::PandapartyApiResult<()> {
    log::debug!("Change my password");
    api::put_no_content("/api/my/password", &ChangeMyPassword { old_password, new_password }).await
}

pub async fn update_my_profile(profile: UpdateProfile) -> api::PandapartyApiResult<()> {
    log::debug!("Update profile to the following data {:?}", profile);
    api::put_no_content("/api/my/profile", &profile).await
}

pub async fn enable_totp() -> api::PandapartyApiResult<TotpQrCode> {
    log::debug!("Enable totp for current user");
    api::post_no_body("/api/my/totp").await
}

pub async fn validate_totp(code: String) -> api::PandapartyApiResult<()> {
    log::debug!("Validate totp for current user");
    api::put_no_content("/api/my/totp/validate", &ValidateTotp {
        code
    }).await
}

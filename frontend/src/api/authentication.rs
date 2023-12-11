use bamboo_entities::prelude::*;

use crate::api;
use crate::storage;

pub async fn login(login_data: Login) -> api::BambooApiResult<either::Either<LoginResult, ()>> {
    log::debug!("Execute login");
    if login_data.two_factor_code.is_none() {
        api::post_no_content("/api/login", &login_data).await?;
        Ok(either::Right(()))
    } else {
        let result = api::post("/api/login", &login_data).await?;
        Ok(either::Left(result))
    }
}

pub fn logout() {
    log::debug!("Execute logout");
    storage::delete_token();
    yew::platform::spawn_local(async {
        let _ = api::delete("/api/login").await;
    });
}

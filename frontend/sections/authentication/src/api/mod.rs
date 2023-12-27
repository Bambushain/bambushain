use bamboo_entities::prelude::*;

use bamboo_frontend_base_api as api;
use bamboo_frontend_base_storage as storage;

pub async fn get_my_profile() -> api::BambooApiResult<WebUser> {
    log::debug!("Get my profile");
    api::get::<WebUser>("/api/my/profile").await.map_err(|err| {
        storage::delete_token();
        err
    })
}

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

pub async fn forgot_password(data: ForgotPassword) -> api::BambooApiResult<()> {
    log::debug!("Request new password");
    api::post_no_content("/api/forgot-password", &data).await
}

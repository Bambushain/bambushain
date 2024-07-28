use bamboo_common::core::entities::*;
use bamboo_common::frontend::api::{ApiError, BambooApiResult};
use bamboo_pandas_frontend_base::*;

pub async fn get_my_profile() -> BambooApiResult<User> {
    log::debug!("Get my profile");
    api::get::<User>("/api/my/profile").await.map_err(|err| {
        storage::delete_token();
        err
    })
}

pub async fn login(login_data: Login) -> BambooApiResult<either::Either<LoginResult, ()>> {
    log::debug!("Execute login");
    let response = api::post_response("/api/login", &login_data).await?;
    Ok(if response.status() == 204 {
        either::Right(())
    } else {
        either::Left(
            serde_json::from_str(response.text().await.unwrap().as_str())
                .map_err(|_| ApiError::json_deserialize_error())?,
        )
    })
}

pub async fn forgot_password(data: ForgotPassword) -> BambooApiResult<()> {
    log::debug!("Request new password");
    api::post_no_content("/api/forgot-password", &data).await
}

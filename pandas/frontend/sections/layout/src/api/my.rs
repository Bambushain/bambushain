use bamboo_common::core::entities::*;
use bamboo_common::frontend::api::BambooApiResult;
use bamboo_pandas_frontend_base::api;

pub async fn change_my_password(old_password: String, new_password: String) -> BambooApiResult<()> {
    log::debug!("Change my password");
    api::put_no_content(
        "/api/my/password",
        &ChangeMyPassword {
            old_password,
            new_password,
        },
    )
    .await
}

pub async fn update_my_profile(profile: UpdateProfile) -> BambooApiResult<()> {
    log::debug!("Update profile to the following data {:?}", profile);
    api::put_no_content("/api/my/profile", &profile).await
}

pub async fn enable_totp() -> BambooApiResult<TotpQrCode> {
    log::debug!("Enable totp for current user");
    api::post_no_body("/api/my/totp").await
}

pub async fn disable_totp() -> BambooApiResult<()> {
    log::debug!("Disable totp for current user");
    api::delete("/api/my/totp").await
}

pub async fn validate_totp(code: String, password: String) -> BambooApiResult<()> {
    log::debug!("Validate totp for current user");
    api::put_no_content("/api/my/totp/validate", &ValidateTotp { code, password }).await
}

pub async fn leave() -> BambooApiResult<()> {
    log::debug!("Leaving the grove");
    api::delete("/api/my").await
}

pub async fn upload_profile_picture(file: web_sys::File) -> BambooApiResult<()> {
    log::debug!("Change profile picture");
    api::upload_file("/api/my/picture", file).await
}

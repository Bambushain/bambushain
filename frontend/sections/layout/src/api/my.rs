use bamboo_entities::prelude::*;
use bamboo_frontend_base_api as api;

pub async fn change_my_password(
    old_password: String,
    new_password: String,
) -> api::BambooApiResult<()> {
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

pub async fn update_my_profile(profile: UpdateProfile) -> api::BambooApiResult<()> {
    log::debug!("Update profile to the following data {:?}", profile);
    api::put_no_content("/api/my/profile", &profile).await
}

pub async fn enable_totp() -> api::BambooApiResult<TotpQrCode> {
    log::debug!("Enable totp for current user");
    api::post_no_body("/api/my/totp").await
}

pub async fn disable_totp() -> api::BambooApiResult<()> {
    log::debug!("Disable totp for current user");
    api::delete("/api/my/totp").await
}

pub async fn validate_totp(code: String, password: String) -> api::BambooApiResult<()> {
    log::debug!("Validate totp for current user");
    api::put_no_content("/api/my/totp/validate", &ValidateTotp { code, password }).await
}

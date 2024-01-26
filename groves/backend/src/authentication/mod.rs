use openidconnect::core::{CoreClient, CoreGenderClaim, CoreProviderMetadata, CoreRevocableToken};
use openidconnect::reqwest::async_http_client;
use openidconnect::{
    AccessToken, AdditionalClaims, ClientId, IssuerUrl, RedirectUrl, UserInfoClaims,
};
use serde::{Deserialize, Serialize};

use bamboo_common::backend::services::EnvService;
use bamboo_common::core::error::{BambooError, BambooErrorResult, BambooResult};

#[derive(Debug, Deserialize, Serialize)]
pub struct ZitadelClaims {
    pub roles: Vec<String>,
}

impl AdditionalClaims for ZitadelClaims {}

pub const ACCESS_TOKEN: &'static str = "access_token";

pub async fn get_client(host: String, env_service: EnvService) -> BambooResult<CoreClient> {
    let provider_metadata = CoreProviderMetadata::discover_async(
        IssuerUrl::new(env_service.get_env("ISSUER_URL", ""))
            .map_err(|err| {
                log::error!("Failed to create issuer url {err}");
                BambooError::unauthorized("login", "Invalid configuration")
            })
            .map_err(|_| BambooError::unauthorized("login", "Invalid configuration"))?,
        async_http_client,
    )
    .await
    .map_err(|_| BambooError::unauthorized("login", "Invalid configuration"))?;

    let client = CoreClient::from_provider_metadata(
        provider_metadata,
        ClientId::new(env_service.get_env("CLIENT_ID", "")),
        None,
    )
    .set_redirect_uri(
        RedirectUrl::new(format!("https://{host}/api/login/callback").to_string())
            .map_err(|_| BambooError::unauthorized("login", "Invalid configuration"))?,
    );

    Ok(client)
}

pub async fn validate_user(access_token: AccessToken, client: CoreClient) -> BambooErrorResult {
    let user_info: UserInfoClaims<ZitadelClaims, CoreGenderClaim> = client
        .user_info(access_token.clone(), None)
        .map_err(|_| BambooError::unauthorized("login", "Invalid user"))?
        .request_async(async_http_client)
        .await
        .map_err(|_| BambooError::unauthorized("login", "Invalid user"))?;

    if !user_info
        .additional_claims()
        .roles
        .contains(&"bambushain-admin".to_string())
    {
        client
            .revoke_token(CoreRevocableToken::from(access_token))
            .map_err(|_| BambooError::unauthorized("login", "Invalid user"))?
            .request_async(async_http_client)
            .await
            .map_err(|_| BambooError::unauthorized("login", "Invalid user"))?;

        Err(BambooError::unauthorized("login", "Token invalid"))
    } else {
        Ok(())
    }
}

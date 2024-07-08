use openidconnect::core::{CoreClient, CoreProviderMetadata, CoreUserInfoClaims};
use openidconnect::reqwest::async_http_client;
use openidconnect::{AccessToken, AdditionalClaims, ClientId, ClientSecret, IssuerUrl, RevocationUrl};
use serde::{Deserialize, Serialize};

use bamboo_common::backend::services::EnvService;
use bamboo_common::core::error::{BambooError, BambooResult};

#[derive(Debug, Deserialize, Serialize)]
pub struct ZitadelClaims {
    pub roles: Vec<String>,
}

impl AdditionalClaims for ZitadelClaims {}

pub async fn get_client(env_service: EnvService) -> BambooResult<CoreClient> {
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

    let mut client = CoreClient::from_provider_metadata(
        provider_metadata,
        ClientId::new(env_service.get_env("CLIENT_ID", "")),
        Some(ClientSecret::new(env_service.get_env("CLIENT_SECRET", ""))),
    );

    if let Some(revocation_url) = env_service.get_env_opt("REVOCATION_URL") {
        if let Ok(revocation_url) = RevocationUrl::new(revocation_url) {
            client = client.set_revocation_uri(revocation_url);
        }
    }

    Ok(client)
}

pub async fn validate_user(access_token: AccessToken, client: CoreClient) -> BambooResult<String> {
    let user_info: CoreUserInfoClaims = client
        .user_info(access_token.clone(), None)
        .map_err(|_| BambooError::unauthorized("login", "Invalid user"))?
        .request_async(async_http_client)
        .await
        .map_err(|_| BambooError::unauthorized("login", "Invalid user"))?;

    user_info
        .name()
        .ok_or(BambooError::unauthorized("user", "The name is required"))
        .map(|name| {
            name.iter()
                .next()
                .map(|(_, name)| name.to_string())
                .ok_or(BambooError::unauthorized("user", "The name is required"))
        })?
}

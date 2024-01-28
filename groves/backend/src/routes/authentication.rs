use actix_session::Session;
use actix_web::web::Redirect;
use actix_web::{get, web};
use actix_web_lab::extract::Host;
use openidconnect::core::{CoreAuthenticationFlow, CoreRevocableToken};
use openidconnect::reqwest::async_http_client;
use openidconnect::{
    AccessToken, AccessTokenHash, AuthorizationCode, CsrfToken, Nonce, OAuth2TokenResponse,
    PkceCodeChallenge, PkceCodeVerifier, Scope, TokenResponse,
};

use bamboo_common::backend::services::EnvService;
use bamboo_common::core::error::{BambooError, BambooResult};

use crate::authentication::{get_client, validate_user, ACCESS_TOKEN};
use crate::middleware::authenticate_user::authenticate;
use crate::query;
use crate::query::LoginCallbackQuery;

const AUTH_PKCE_VERIFIER: &'static str = "auth-pkce-verifier";
const AUTH_CSRF_TOKEN: &'static str = "auth-csrf-token";
const AUTH_NONCE: &'static str = "auth-nonce";

#[get("/api/login")]
pub async fn login(
    session: Session,
    env_service: EnvService,
    host: Host,
) -> Result<Redirect, BambooError> {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (auth_url, csrf_token, nonce) = get_client(host.into(), env_service)
        .await?
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("roles".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    session.insert(AUTH_CSRF_TOKEN, csrf_token).map_err(|err| {
        log::error!("{AUTH_CSRF_TOKEN}: {err}");
        BambooError::unauthorized("login", "Invalid session")
    })?;
    session.insert(AUTH_NONCE, nonce).map_err(|err| {
        log::error!("{AUTH_NONCE}: {err}");
        BambooError::unauthorized("login", "Invalid session")
    })?;
    session
        .insert(AUTH_PKCE_VERIFIER, pkce_verifier)
        .map_err(|err| {
            log::error!("{AUTH_PKCE_VERIFIER}: {err}");
            BambooError::unauthorized("login", "Invalid session")
        })?;

    session.renew();
    let redirect_url = auth_url.to_string();

    Ok(Redirect::to(redirect_url).see_other())
}

#[get("/api/login/callback")]
pub async fn login_callback(
    callback: web::Query<query::LoginCallbackQuery>,
    session: Session,
    env_service: EnvService,
    host: Host,
) -> BambooResult<Redirect> {
    let result =
        perform_login_callback(callback.into_inner(), session.clone(), env_service, host).await;
    if result.is_err() {
        session.purge();
        Ok(Redirect::to("/"))
    } else {
        result
    }
}

async fn perform_login_callback(
    callback: LoginCallbackQuery,
    session: Session,
    env_service: EnvService,
    host: Host,
) -> BambooResult<Redirect> {
    let csrf_token = session
        .get::<CsrfToken>(AUTH_CSRF_TOKEN)
        .map_err(|err| {
            log::error!("{AUTH_CSRF_TOKEN}: {err}");
            BambooError::unauthorized("login", "Invalid session")
        })?
        .ok_or_else(|| BambooError::unauthorized("login", "Invalid session"))?;
    let nonce = session
        .get::<Nonce>(AUTH_NONCE)
        .map_err(|err| {
            log::error!("{AUTH_NONCE}: {err}");
            BambooError::unauthorized("login", "Invalid session")
        })?
        .ok_or_else(|| BambooError::unauthorized("login", "Invalid session"))?;
    let pkce_verifier = session
        .get::<PkceCodeVerifier>(AUTH_PKCE_VERIFIER)
        .map_err(|err| {
            log::error!("{AUTH_PKCE_VERIFIER}: {err}");
            BambooError::unauthorized("login", "Invalid session")
        })?
        .ok_or_else(|| BambooError::unauthorized("login", "Invalid session"))?;

    let _ = session.remove(AUTH_CSRF_TOKEN);
    let _ = session.remove(AUTH_NONCE);
    let _ = session.remove(AUTH_PKCE_VERIFIER);

    if callback.state == csrf_token.secret().to_string() {
        let client = get_client(host.into(), env_service.clone()).await?;
        let token_response = client
            .exchange_code(AuthorizationCode::new(callback.code.clone().into()))
            .set_pkce_verifier(pkce_verifier)
            .request_async(async_http_client)
            .await
            .map_err(|err| {
                log::error!("Failed to check code: {err}");
                BambooError::unauthorized("login", "Invalid user")
            })?;

        let id_token = token_response.id_token().ok_or_else(|| {
            BambooError::unauthorized("login", "Server did not return an ID token")
        })?;

        let mut valid_audiences = env_service
            .get_env("ADDITIONAL_AUDIENCES", "")
            .split(",")
            .map(String::from)
            .collect::<Vec<String>>();
        valid_audiences.push(env_service.get_env("CLIENT_ID", ""));

        let verifier = client
            .id_token_verifier()
            .set_other_audience_verifier_fn(|aud| valid_audiences.contains(aud));
        let claims = id_token.claims(&verifier, &nonce).map_err(|err| {
            log::error!("Failed to get id token claims: {err}");
            BambooError::unauthorized("login", "Invalid callback data")
        })?;

        if let Some(expected_access_token_hash) = claims.access_token_hash() {
            let actual_access_token_hash = AccessTokenHash::from_token(
                token_response.access_token(),
                &id_token
                    .signing_alg()
                    .map_err(|_| BambooError::unauthorized("login", "Invalid access token"))?,
            )
            .map_err(|_| BambooError::unauthorized("login", "Invalid access token"))?;
            if actual_access_token_hash != *expected_access_token_hash {
                return Err(BambooError::unauthorized("login", "Invalid access token"));
            }
        }

        let access_token = token_response.access_token().clone();
        let _ = session.insert(ACCESS_TOKEN, access_token.clone());

        validate_user(access_token, client.clone()).await?;

        Ok(Redirect::to("/"))
    } else {
        session.purge();
        Err(BambooError::unauthorized("login", "Invalid callback data"))
    }
}

#[get("/api/logout", wrap = "authenticate!()")]
pub async fn logout(
    session: Session,
    env_service: EnvService,
    host: Host,
) -> BambooResult<Redirect> {
    let access_token = session
        .get::<AccessToken>(ACCESS_TOKEN)
        .map_err(|err| {
            log::error!("{ACCESS_TOKEN}: {err}");
            BambooError::unauthorized("login", "Invalid session")
        })?
        .ok_or_else(|| BambooError::unauthorized("login", "Invalid session"))?;
    let _ = get_client(host.into(), env_service)
        .await?
        .revoke_token(CoreRevocableToken::from(access_token))
        .map_err(|err| log::error!("Revoke failed: {err}"));
    session.purge();

    Ok(Redirect::to("/").see_other())
}

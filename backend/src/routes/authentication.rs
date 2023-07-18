use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use sheef_database::token::{remove_token, validate_auth_and_create_token};
use sheef_entities::{Login, SheefErrorCode};
use crate::middleware::authenticate_user::AuthenticationState;

pub async fn login(body: web::Json<Login>) -> HttpResponse {
    let data = validate_auth_and_create_token(&body.username, &body.password).await;
    if let Ok(result) = data {
        ok_json!(result)
    } else {
        HttpResponse::Unauthorized().json(sheef_entities::SheefError {
            entity_type: "user".to_string(),
            message: "Username or Password is invalid".to_string(),
            error_type: SheefErrorCode::InvalidDataError,
        })
    }
}

pub async fn logout(req: HttpRequest) -> HttpResponse {
    let (username, token) = {
        let extensions = req.extensions();
        let state = extensions.get::<AuthenticationState>();
        if state.is_none() {
            return no_content!();
        }

        let result = state.unwrap();
        (result.user.username.to_string(), result.token.to_string())
    };

    remove_token(&username, &token).await;

    no_content!()
}
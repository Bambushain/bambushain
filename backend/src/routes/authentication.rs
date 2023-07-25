use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};

use sheef_dbal::prelude::*;
use sheef_entities::prelude::*;

use crate::middleware::authenticate_user::AuthenticationState;

pub async fn login(body: web::Json<Login>) -> HttpResponse {
    let data = validate_auth_and_create_token(body.username.clone(), body.password.clone()).await;
    if let Ok(result) = data {
        ok_json!(result)
    } else {
        HttpResponse::Unauthorized().json(SheefError {
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

    let _ = delete_token(username, token).await;

    no_content!()
}
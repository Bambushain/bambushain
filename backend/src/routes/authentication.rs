use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use actix_web::http::StatusCode;
use sheef_database::token::{remove_token, validate_auth_and_create_token};
use sheef_entities::Login;
use crate::middleware::authenticate_user::AuthenticationState;

pub async fn login(body: web::Json<Login>) -> HttpResponse {
    let data = web::block(move || validate_auth_and_create_token(&body.username, &body.password)).await;
    if let Ok(Some(result)) = data {
        ok_json!(result)
    } else {
        HttpResponse::new(StatusCode::UNAUTHORIZED)
    }
}

pub async fn logout(req: HttpRequest) -> HttpResponse {
    let (username, token) = {
        let extensions = req.extensions();
        let state = extensions.get::<AuthenticationState>();
        if state.is_none() {
            return HttpResponse::new(StatusCode::NO_CONTENT);
        }

        let result = state.unwrap();
        (result.user.username.to_string(), result.token.to_string())
    };

    remove_token(&username, &token);

    no_content!()
}
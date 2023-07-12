use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use actix_web::http::StatusCode;
use sheef_database::token::{remove_token, validate_auth_and_create_token};
use sheef_entities::Login;
use crate::middleware::authenticate_user::AuthenticationState;

pub async fn login(body: web::Json<Login>) -> HttpResponse {
    let data = web::block(move || validate_auth_and_create_token(&body.username, &body.password)).await;
    match data {
        Ok(result) => match result {
            Some(result) => HttpResponse::Ok().json(web::Json(result)),
            None => HttpResponse::new(StatusCode::NOT_FOUND),
        }
        Err(_) => HttpResponse::new(StatusCode::NOT_FOUND)
    }
}

pub async fn logout(_req: HttpRequest) -> HttpResponse {
    let (username, token) = {
        let extensions = _req.extensions();
        let state = extensions.get::<AuthenticationState>();
        if state.is_none() {
            return HttpResponse::new(StatusCode::NO_CONTENT);
        }

        let result = state.unwrap();
        (result.user.username.to_string(), result.token.to_string())
    };

    remove_token(&username, &token);

    HttpResponse::new(StatusCode::NO_CONTENT)
}
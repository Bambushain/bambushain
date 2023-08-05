use actix_web::{HttpResponse, web};

use pandaparty_dbal::prelude::*;
use pandaparty_entities::prelude::*;
use crate::DbConnection;

use crate::middleware::authenticate_user::AuthenticationState;

pub async fn login(body: web::Json<Login>, db: DbConnection) -> HttpResponse {
    let data = validate_auth_and_create_token(body.username.clone(), body.password.clone(), &db).await;
    match data {
        Ok(result) => ok_json!(result),
        Err(err) => {
            log::error!("Failed to login {err}");
            HttpResponse::Unauthorized().json(PandaPartyError {
                entity_type: "user".to_string(),
                message: "Username or Password is invalid".to_string(),
                error_type: PandaPartyErrorCode::InvalidDataError,
            })
        }
    }
}

pub async fn logout(state: web::ReqData<AuthenticationState>, db: DbConnection) -> HttpResponse {
    let _ = delete_token(state.token.clone(), &db).await;

    no_content!()
}
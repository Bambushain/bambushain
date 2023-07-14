use std::rc::Rc;
use sheef_entities::Login;
use crate::api::{ErrorCode, post};

pub async fn login(login_data: Rc<Login>) -> Result<sheef_entities::authentication::LoginResult, ErrorCode> {
    log::debug!("Execute login");
    post("/api/login", login_data).await
}
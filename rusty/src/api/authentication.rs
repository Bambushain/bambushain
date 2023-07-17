use std::ops::Deref;
use std::rc::Rc;
use sheef_entities::Login;
use crate::api::{delete, ErrorCode, post};
use crate::storage::delete_token;

pub async fn login(login_data: Rc<Login>) -> Result<sheef_entities::authentication::LoginResult, ErrorCode> {
    log::debug!("Execute login");
    post("/api/login", login_data.deref()).await
}

pub fn logout() {
    log::debug!("Execute logout");
    delete_token();
    yew::platform::spawn_local(async {
        delete("/api/login").await;
    });
}

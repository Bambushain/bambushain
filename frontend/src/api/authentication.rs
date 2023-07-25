use std::ops::Deref;
use std::rc::Rc;

use sheef_entities::prelude::*;

use crate::api::{delete, post, SheefApiResult};
use crate::storage::delete_token;

pub async fn login(login_data: Rc<Login>) -> SheefApiResult<LoginResult> {
    log::debug!("Execute login");
    post("/api/login", login_data.deref()).await
}

pub fn logout() {
    log::debug!("Execute logout");
    delete_token();
    yew::platform::spawn_local(async {
        let _ = delete("/api/login").await;
    });
}

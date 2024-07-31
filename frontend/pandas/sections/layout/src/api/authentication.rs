use bamboo_frontend_pandas_base::{api, storage};

pub fn logout() {
    log::debug!("Execute logout");
    storage::delete_token();
    yew::platform::spawn_local(async {
        let _ = api::delete("/api/login").await;
    });
}

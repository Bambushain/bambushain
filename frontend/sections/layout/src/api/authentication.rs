use bamboo_frontend_base_api as api;
use bamboo_frontend_base_storage as storage;

pub fn logout() {
    log::debug!("Execute logout");
    storage::delete_token();
    yew::platform::spawn_local(async {
        let _ = api::delete("/api/login").await;
    });
}

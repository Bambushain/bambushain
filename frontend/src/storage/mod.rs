use bamboo_entities::user::WebUser;
use bounce::Atom;
use gloo::storage::{LocalStorage, Storage};

pub fn get_token() -> Option<String> {
    LocalStorage::get("/bamboo/token").ok()
}

pub fn set_token(token: String) {
    _ = LocalStorage::set("/bamboo/token", token);
}

pub fn delete_token() {
    LocalStorage::delete("/bamboo/token");
}

pub fn get_log_level() -> Option<String> {
    LocalStorage::get("/bamboo/log/level").ok()
}

pub fn is_logging_on() -> bool {
    get_log_level().is_some()
}

#[derive(Atom, PartialEq, Clone, Default)]
pub struct CurrentUser {
    pub profile: WebUser,
}

impl From<WebUser> for CurrentUser {
    fn from(value: WebUser) -> Self {
        Self { profile: value }
    }
}

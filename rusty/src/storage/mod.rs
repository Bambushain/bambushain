use bounce::Atom;
use gloo::storage::{LocalStorage, Storage};

pub fn get_token() -> Option<String> {
    LocalStorage::get("/sheef/token").ok()
}

pub fn set_token(token: String) {
    _ = LocalStorage::set("/sheef/token", token);
}

pub fn delete_token() {
    LocalStorage::delete("/sheef/token");;
}

pub fn get_log_level() -> Option<String> {
    LocalStorage::get("/sheef/log/level").ok()
}

pub fn is_logging_on() -> bool {
    get_log_level().is_some()
}

#[derive(Atom, PartialEq, Clone, Default)]
pub struct CurrentUser {
    pub profile: sheef_entities::User,
}

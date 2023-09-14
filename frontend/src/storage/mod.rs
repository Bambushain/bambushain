use bounce::Atom;
use gloo::storage::{LocalStorage, Storage};
use pandaparty_entities::user::WebUser;

pub fn get_token() -> Option<String> {
    LocalStorage::get("/pandaparty/token").ok()
}

pub fn set_token(token: String) {
    _ = LocalStorage::set("/pandaparty/token", token);
}

pub fn delete_token() {
    LocalStorage::delete("/pandaparty/token");
}

pub fn get_log_level() -> Option<String> {
    LocalStorage::get("/pandaparty/log/level").ok()
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

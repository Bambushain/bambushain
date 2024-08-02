use bamboo_common::core::entities::User;
use bounce::Atom;
use gloo_storage::{LocalStorage, Storage};

pub fn set_token(token: String) {
    _ = LocalStorage::set("/bamboo/token", token);
}

pub fn delete_token() {
    LocalStorage::delete("/bamboo/token");
}

pub fn get_log_level() -> Option<String> {
    LocalStorage::get("/bamboo/log/level").ok()
}

#[derive(Atom, PartialEq, Clone, Default)]
pub struct CurrentUser {
    pub profile: User,
}

impl From<User> for CurrentUser {
    fn from(value: User) -> Self {
        Self { profile: value }
    }
}

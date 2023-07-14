use std::fmt::{Display, Formatter};
use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/sheef")]
    #[not_found]
    Sheef,
    #[at("/sheef/*")]
    SheefRoot,
    #[at("/login")]
    Login,
}

impl Display for AppRoute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_path().as_str())
    }
}

#[derive(Clone, Routable, PartialEq)]
pub enum SheefRoute {
    #[at("/calendar")]
    #[not_found]
    Calendar,
}
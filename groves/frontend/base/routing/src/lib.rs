use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    #[not_found]
    Home,
    #[at("/app")]
    Groves,
    #[at("/app/mods/:grove_id")]
    Users { grove_id: i32 },
}

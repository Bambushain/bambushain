use std::fmt::{Display, Formatter};

use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    #[not_found]
    Home,
    #[at("/pandaparty")]
    PandaPartyRoot,
    #[at("/pandaparty/*")]
    PandaParty,
    #[at("/final-fantasy")]
    FinalFantasyRoot,
    #[at("/final-fantasy/*")]
    FinalFantasy,
    #[at("/login")]
    Login,
}

impl Display for AppRoute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_path().as_str())
    }
}

#[derive(Clone, Routable, PartialEq)]
pub enum FinalFantasyRoute {
    #[at("/final-fantasy/character")]
    Characters,
}

#[derive(Clone, Routable, PartialEq)]
pub enum PandaPartyRoute {
    #[at("/pandaparty/calendar")]
    Calendar,
    #[at("/pandaparty/user")]
    User,
}

impl Display for FinalFantasyRoute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_path().as_str())
    }
}

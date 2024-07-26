use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/pandas")]
    #[not_found]
    Home,
    #[at("/pandas/bamboo")]
    BambooGroveRoot,
    #[at("/pandas/bamboo/*")]
    BambooGrove,
    #[at("/pandas/groves")]
    GrovesRoot,
    #[at("/pandas/groves/*")]
    Groves,
    #[at("/pandas/final-fantasy")]
    FinalFantasyRoot,
    #[at("/pandas/final-fantasy/*")]
    FinalFantasy,
    #[at("/pandas/support")]
    SupportRoot,
    #[at("/pandas/support/*")]
    Support,
    #[at("/pandas/legal")]
    LegalRoot,
    #[at("/pandas/legal/*")]
    Legal,
    #[at("/pandas/licenses")]
    LicensesRoot,
    #[at("/pandas/licenses/*")]
    Licenses,
    #[at("/pandas/login")]
    Login,
}

#[derive(Clone, Routable, PartialEq)]
pub enum FinalFantasyRoute {
    #[at("/pandas/final-fantasy")]
    Characters,
    #[at("/pandas/final-fantasy/settings")]
    Settings,
}

#[derive(Clone, Routable, PartialEq)]
pub enum SupportRoute {
    #[at("/pandas/support")]
    Contact,
}

#[derive(Clone, Routable, PartialEq)]
pub enum BambooGroveRoute {
    #[at("/pandas/bamboo")]
    Calendar,
    #[at("/pandas/bamboo/user")]
    User,
}

#[derive(Clone, Routable, PartialEq)]
pub enum GroveRoute {
    #[at("/pandas/groves/add")]
    AddGrove,
    #[at("/pandas/groves/:id/:name")]
    Grove { id: i32, name: String },
    #[at("/pandas/groves/:id/:name/:invite_secret")]
    GroveInvite {
        id: i32,
        name: String,
        invite_secret: String,
    },
}

#[derive(Clone, Routable, PartialEq)]
pub enum LegalRoute {
    #[at("/pandas/legal")]
    Imprint,
    #[at("/pandas/legal/data-protection")]
    DataProtection,
}

#[derive(Clone, Routable, PartialEq)]
pub enum LicensesRoute {
    #[at("/pandas/licenses")]
    BambooGrove,
    #[at("/pandas/licenses/images")]
    Images,
    #[at("/pandas/licenses/fonts")]
    Fonts,
    #[at("/pandas/licenses/software")]
    Software,
}

use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    #[not_found]
    Home,
    #[at("/bamboo")]
    BambooGroveRoot,
    #[at("/bamboo/*")]
    BambooGrove,
    #[at("/groves")]
    GrovesRoot,
    #[at("/groves/*")]
    Groves,
    #[at("/final-fantasy")]
    FinalFantasyRoot,
    #[at("/final-fantasy/*")]
    FinalFantasy,
    #[at("/support")]
    SupportRoot,
    #[at("/support/*")]
    Support,
    #[at("/legal")]
    LegalRoot,
    #[at("/legal/*")]
    Legal,
    #[at("/licenses")]
    LicensesRoot,
    #[at("/licenses/*")]
    Licenses,
    #[at("/login")]
    Login,
}

#[derive(Clone, Routable, PartialEq)]
pub enum FinalFantasyRoute {
    #[at("/final-fantasy")]
    Characters,
    #[at("/final-fantasy/settings")]
    Settings,
}

#[derive(Clone, Routable, PartialEq)]
pub enum SupportRoute {
    #[at("/support")]
    Contact,
}

#[derive(Clone, Routable, PartialEq)]
pub enum BambooGroveRoute {
    #[at("/bamboo")]
    Calendar,
    #[at("/bamboo/user")]
    User,
}

#[derive(Clone, Routable, PartialEq)]
pub enum GroveRoute {
    #[at("/groves/add")]
    AddGrove,
    #[at("/groves/:id")]
    Grove { id: i32 },
}

#[derive(Clone, Routable, PartialEq)]
pub enum LegalRoute {
    #[at("/legal")]
    Imprint,
    #[at("/legal/data-protection")]
    DataProtection,
}

#[derive(Clone, Routable, PartialEq)]
pub enum LicensesRoute {
    #[at("/licenses")]
    BambooGrove,
    #[at("/licenses/images")]
    Images,
    #[at("/licenses/fonts")]
    Fonts,
    #[at("/licenses/software")]
    Software,
}

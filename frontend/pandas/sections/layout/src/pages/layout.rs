use bounce::helmet::Helmet;
use bounce::{use_atom_setter, use_atom_value};
use yew::prelude::*;
use yew_cosmo::prelude::*;
use yew_hooks::{use_async, use_mount};
use yew_router::prelude::*;

use bamboo_frontend_pandas_base::routing::{
    AppRoute, BambooGroveRoute, FinalFantasyRoute, GroveRoute, LegalRoute, LicensesRoute,
    MyProfileRoute, SupportRoute,
};
use bamboo_frontend_pandas_base::storage;
use bamboo_frontend_pandas_section_authentication::{LoginPage, ResetPasswordPage};
use bamboo_frontend_pandas_section_bamboo::CalendarPage;
use bamboo_frontend_pandas_section_bamboo::UsersPage;
use bamboo_frontend_pandas_section_final_fantasy::CharacterPage;
use bamboo_frontend_pandas_section_final_fantasy::SettingsPage;
use bamboo_frontend_pandas_section_groves::pages::groves::{
    AddGrovePage, GroveDetailsPage, GroveInvitePage,
};
use bamboo_frontend_pandas_section_groves::use_groves;
use bamboo_frontend_pandas_section_legal::{DataProtectionPage, ImprintPage};
use bamboo_frontend_pandas_section_licenses::{
    BambooGrovePage, FontsPage, ImagesPage, SoftwareLicensesPage,
};
use bamboo_frontend_pandas_section_profile::MyProfilePage;
use bamboo_frontend_pandas_section_support::ContactPage;

use crate::api;

pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Login => html!(
            <>
                <Helmet>
                    <title>{ "Anmelden" }</title>
                </Helmet>
                <LoginPage />
            </>
        ),
        AppRoute::ResetPassword => html!(
            <>
                <Helmet>
                    <title>{ "Passwort zurücksetzen" }</title>
                </Helmet>
                <ResetPasswordPage />
            </>
        ),
        _ => html!(<Layout />),
    }
}

#[function_component(GrovesMenu)]
fn groves_menus() -> Html {
    let groves_atom = use_groves();

    html!(
        { for (*groves_atom).groves.iter().cloned().map(|grove| {
            let name = grove.name.clone();

            html!(
                <Switch<GroveRoute> render={render_sub_menu_entry(name.clone(), GroveRoute::Grove { id: grove.id, name: grove.name })} />
            )
        }) }
    )
}

#[function_component(GrovesRoot)]
fn groves_root() -> Html {
    let groves_atom = use_groves();

    let to = if let Some(first) = (*groves_atom).groves.first() {
        GroveRoute::Grove {
            id: first.id,
            name: first.name.clone(),
        }
    } else {
        GroveRoute::AddGrove
    };

    html!(<Redirect<GroveRoute> to={to} />)
}

fn switch_sub_menu(route: AppRoute) -> Html {
    match route {
        AppRoute::BambooGroveRoot | AppRoute::BambooGrove => html!(
            <CosmoSubMenuBar>
                <Switch<BambooGroveRoute>
                    render={render_sub_menu_entry("Event Kalender", BambooGroveRoute::Calendar)}
                />
                <Switch<BambooGroveRoute>
                    render={render_sub_menu_entry("Pandas", BambooGroveRoute::User)}
                />
            </CosmoSubMenuBar>
        ),
        AppRoute::FinalFantasyRoot | AppRoute::FinalFantasy => html!(
            <CosmoSubMenuBar>
                <Switch<FinalFantasyRoute>
                    render={render_sub_menu_entry("Meine Charaktere", FinalFantasyRoute::Characters)}
                />
                <Switch<FinalFantasyRoute>
                    render={render_sub_menu_entry("Personalisierung", FinalFantasyRoute::Settings)}
                />
            </CosmoSubMenuBar>
        ),
        AppRoute::GrovesRoot | AppRoute::Groves => html!(
            <CosmoSubMenuBar>
                <GrovesMenu />
                <Switch<GroveRoute>
                    render={render_sub_menu_entry("Neuer Hain", GroveRoute::AddGrove)}
                />
            </CosmoSubMenuBar>
        ),
        AppRoute::MyProfileRoot | AppRoute::MyProfile => html!(
            <CosmoSubMenuBar>
                <Switch<MyProfileRoute>
                    render={render_sub_menu_entry("Über mich", MyProfileRoute::MyProfile)}
                />
            </CosmoSubMenuBar>
        ),
        AppRoute::LegalRoot | AppRoute::Legal => html!(
            <CosmoSubMenuBar>
                <Switch<LegalRoute>
                    render={render_sub_menu_entry("Impressum", LegalRoute::Imprint)}
                />
                <Switch<LegalRoute>
                    render={render_sub_menu_entry("Datenschutzerklärung", LegalRoute::DataProtection)}
                />
            </CosmoSubMenuBar>
        ),
        AppRoute::LicensesRoot | AppRoute::Licenses => html!(
            <CosmoSubMenuBar>
                <Switch<LicensesRoute>
                    render={render_sub_menu_entry("Bambushain Lizenz", LicensesRoute::BambooGrove)}
                />
                <Switch<LicensesRoute>
                    render={render_sub_menu_entry("Bildlizenzen", LicensesRoute::Images)}
                />
                <Switch<LicensesRoute>
                    render={render_sub_menu_entry("Schriftlizenzen", LicensesRoute::Fonts)}
                />
                <Switch<LicensesRoute>
                    render={render_sub_menu_entry("Softwarelizenzen", LicensesRoute::Software)}
                />
            </CosmoSubMenuBar>
        ),
        _ => {
            log::debug!("Other");
            html!()
        }
    }
}

fn switch_main_menu(route: AppRoute) -> Html {
    match route {
        AppRoute::Home
        | AppRoute::BambooGroveRoot
        | AppRoute::BambooGrove
        | AppRoute::GrovesRoot
        | AppRoute::Groves
        | AppRoute::FinalFantasyRoot
        | AppRoute::FinalFantasy
        | AppRoute::MyProfileRoot
        | AppRoute::MyProfile => {
            html! {
                <CosmoMainMenu>
                    <Switch<AppRoute>
                        render={render_main_menu_entry("Bambushain", AppRoute::BambooGroveRoot, AppRoute::BambooGrove)}
                    />
                    <Switch<AppRoute>
                        render={render_main_menu_entry("Final Fantasy", AppRoute::FinalFantasyRoot, AppRoute::FinalFantasy)}
                    />
                    <Switch<AppRoute>
                        render={render_main_menu_entry("Meine Haine", AppRoute::GrovesRoot, AppRoute::Groves)}
                    />
                    <Switch<AppRoute>
                        render={render_main_menu_entry("Mein Profil", AppRoute::MyProfileRoot, AppRoute::MyProfile)}
                    />
                </CosmoMainMenu>
            }
        }
        AppRoute::SupportRoot | AppRoute::Support => {
            html! {
                <CosmoMainMenu>
                    <Switch<AppRoute>
                        render={render_main_menu_entry("Bambushain", AppRoute::BambooGroveRoot, AppRoute::BambooGrove)}
                    />
                    <Switch<AppRoute>
                        render={render_main_menu_entry("Final Fantasy", AppRoute::FinalFantasyRoot, AppRoute::FinalFantasy)}
                    />
                    <Switch<AppRoute>
                        render={render_main_menu_entry("Meine Haine", AppRoute::GrovesRoot, AppRoute::Groves)}
                    />
                    <Switch<AppRoute>
                        render={render_main_menu_entry("Mein Profil", AppRoute::MyProfileRoot, AppRoute::MyProfile)}
                    />
                    <Switch<AppRoute>
                        render={render_main_menu_entry("Bambussupport", AppRoute::SupportRoot, AppRoute::Support)}
                    />
                </CosmoMainMenu>
            }
        }
        _ => {
            html! {}
        }
    }
}

fn switch_final_fantasy(route: FinalFantasyRoute) -> Html {
    match route {
        FinalFantasyRoute::Characters => html!(
            <>
                <Helmet>
                    <title>{ "Meine Charaktere" }</title>
                </Helmet>
                <CharacterPage />
            </>
        ),
        FinalFantasyRoute::Settings => html!(
            <>
                <Helmet>
                    <title>{ "Personalisierung" }</title>
                </Helmet>
                <SettingsPage />
            </>
        ),
    }
}

fn switch_groves(route: GroveRoute) -> Html {
    match route {
        GroveRoute::AddGrove => html!(<AddGrovePage />),
        GroveRoute::Grove { id, name } => html!(<GroveDetailsPage id={id} name={name} />),
        GroveRoute::GroveInvite {
            id,
            name,
            invite_secret,
        } => html!(<GroveInvitePage id={id} name={name} invite_secret={invite_secret} />),
    }
}

fn switch_bamboo_grove(route: BambooGroveRoute) -> Html {
    match route {
        BambooGroveRoute::Calendar => html!(
            <>
                <Helmet>
                    <title>{ "Event Kalender" }</title>
                </Helmet>
                <CalendarPage />
            </>
        ),
        BambooGroveRoute::User => html!(
            <>
                <Helmet>
                    <title>{ "Pandas" }</title>
                </Helmet>
                <UsersPage />
            </>
        ),
    }
}

fn switch_my_profile(route: MyProfileRoute) -> Html {
    match route {
        MyProfileRoute::MyProfile => html!(<MyProfilePage />),
    }
}

fn switch_support(route: SupportRoute) -> Html {
    match route {
        SupportRoute::Contact => html!(
            <>
                <Helmet>
                    <title>{ "Kontakt" }</title>
                </Helmet>
                <ContactPage />
            </>
        ),
    }
}

fn switch_legal(route: LegalRoute) -> Html {
    match route {
        LegalRoute::Imprint => html!(
            <>
                <Helmet>
                    <title>{ "Impressum" }</title>
                </Helmet>
                <ImprintPage />
            </>
        ),
        LegalRoute::DataProtection => html!(
            <>
                <Helmet>
                    <title>{ "Datenschutzerklärung" }</title>
                </Helmet>
                <DataProtectionPage />
            </>
        ),
    }
}

fn switch_licenses(route: LicensesRoute) -> Html {
    match route {
        LicensesRoute::BambooGrove => html!(
            <>
                <Helmet>
                    <title>{ "Bambushain Lizenz" }</title>
                </Helmet>
                <BambooGrovePage />
            </>
        ),
        LicensesRoute::Images => html!(
            <>
                <Helmet>
                    <title>{ "Bildlizenzen" }</title>
                </Helmet>
                <ImagesPage />
            </>
        ),
        LicensesRoute::Fonts => html!(
            <>
                <Helmet>
                    <title>{ "Schriftlizenzen" }</title>
                </Helmet>
                <FontsPage />
            </>
        ),
        LicensesRoute::Software => html!(
            <>
                <Helmet>
                    <title>{ "Softwarelizenzen" }</title>
                </Helmet>
                <SoftwareLicensesPage />
            </>
        ),
    }
}

fn switch_app(route: AppRoute) -> Html {
    match route {
        AppRoute::Home => html!(<Redirect<AppRoute> to={AppRoute::BambooGroveRoot} />),
        AppRoute::BambooGroveRoot | AppRoute::BambooGrove => html!(
            <>
                <Helmet>
                    <title>{ "Bambushain" }</title>
                </Helmet>
                <Switch<BambooGroveRoute> render={switch_bamboo_grove} />
            </>
        ),
        AppRoute::FinalFantasyRoot | AppRoute::FinalFantasy => html!(
            <>
                <Helmet>
                    <title>{ "Final Fantasy" }</title>
                </Helmet>
                <Switch<FinalFantasyRoute> render={switch_final_fantasy} />
            </>
        ),
        AppRoute::GrovesRoot => html!(<GrovesRoot />),
        AppRoute::Groves => html!(
            <>
                <Helmet>
                    <title>{ "Meine Haine" }</title>
                </Helmet>
                <Switch<GroveRoute> render={switch_groves} />
            </>
        ),
        AppRoute::MyProfileRoot | AppRoute::MyProfile => html!(
            <>
                <Helmet>
                    <title>{ "Mein Profil" }</title>
                </Helmet>
                <Switch<MyProfileRoute> render={switch_my_profile} />
            </>
        ),
        AppRoute::SupportRoot | AppRoute::Support => html!(
            <>
                <Helmet>
                    <title>{ "Bambussupport" }</title>
                </Helmet>
                <Switch<SupportRoute> render={switch_support} />
            </>
        ),
        AppRoute::LegalRoot | AppRoute::Legal => html!(
            <>
                <Helmet>
                    <title>{ "Rechtliches" }</title>
                </Helmet>
                <Switch<LegalRoute> render={switch_legal} />
            </>
        ),
        AppRoute::LicensesRoot | AppRoute::Licenses => html!(
            <>
                <Helmet>
                    <title>{ "Lizenz" }</title>
                </Helmet>
                <Switch<LicensesRoute> render={switch_licenses} />
            </>
        ),
        AppRoute::Login | AppRoute::ResetPassword => html!(),
    }
}

fn render_main_menu_entry(
    label: impl Into<AttrValue> + Clone,
    to: AppRoute,
    active: AppRoute,
) -> impl Fn(AppRoute) -> Html {
    move |route| {
        let is_active = route.eq(&active) || route.eq(&to);

        html!(
            <CosmoMainMenuItemLink<AppRoute>
                to={to.clone()}
                label={label.clone().into()}
                is_active={is_active}
            />
        )
    }
}

fn render_sub_menu_entry<Route: Routable + Clone + 'static>(
    label: impl Into<AttrValue> + Clone,
    to: Route,
) -> impl Fn(Route) -> Html {
    move |route| {
        let is_active = route.eq(&to);

        html!(
            <CosmoSubMenuItemLink<Route>
                to={to.clone()}
                label={label.clone().into()}
                is_active={is_active}
            />
        )
    }
}

fn switch_top_bar(route: AppRoute) -> Html {
    match route {
        AppRoute::Login => html!(),
        AppRoute::LegalRoot | AppRoute::Legal | AppRoute::LicensesRoot | AppRoute::Licenses => {
            html!(<TopBarLegal />)
        }
        _ => html!(<TopBar />),
    }
}

fn switch_layout(route: AppRoute) -> Html {
    match route {
        AppRoute::LegalRoot | AppRoute::Legal | AppRoute::LicensesRoot | AppRoute::Licenses => {
            html!(<LegalLayout />)
        }
        _ => html!(<AppLayout />),
    }
}

#[function_component(AppLayout)]
fn app_layout() -> Html {
    log::debug!("Render app layout");
    let profile_atom_setter = use_atom_setter::<storage::CurrentUser>();

    let profile_state = use_async(async move {
        api::get_my_profile().await.map(|user| {
            profile_atom_setter(user.clone().into());
            user
        })
    });

    {
        let profile_state = profile_state.clone();

        use_mount(move || {
            log::debug!(
                "First render, so lets send the request to check if the token is valid and see"
            );
            profile_state.run();
        });
    }

    html!(
        if let Some(_) = &profile_state.error {
            <Redirect<AppRoute> to={AppRoute::Login} />
        } else if profile_state.data.is_some() {
            <>
                <Switch<AppRoute> render={switch_top_bar} />
                <CosmoMenuBar>
                    <Switch<AppRoute> render={switch_main_menu} />
                    <Switch<AppRoute> render={switch_sub_menu} />
                </CosmoMenuBar>
                <CosmoPageBody>
                    <Switch<AppRoute> render={switch_app} />
                </CosmoPageBody>
            </>
        }
    )
}

#[function_component(LegalLayout)]
fn legal_layout() -> Html {
    log::debug!("Render legal layout");
    html!(
        <>
            <Switch<AppRoute> render={switch_top_bar} />
            <CosmoMenuBar>
                <CosmoMainMenu>
                    <Switch<AppRoute>
                        render={render_main_menu_entry("Rechtliches", AppRoute::LegalRoot, AppRoute::Legal)}
                    />
                    <Switch<AppRoute>
                        render={render_main_menu_entry("Lizenzen", AppRoute::LicensesRoot, AppRoute::Licenses)}
                    />
                </CosmoMainMenu>
                <Switch<AppRoute> render={switch_sub_menu} />
            </CosmoMenuBar>
            <CosmoPageBody>
                <Switch<AppRoute> render={switch_app} />
            </CosmoPageBody>
        </>
    )
}

#[function_component(TopBar)]
fn top_bar() -> Html {
    log::debug!("Render top bar");
    let profile_atom = use_atom_value::<storage::CurrentUser>();

    let profile_user_id = use_state(|| profile_atom.profile.id);

    let navigator = use_navigator().expect("Navigator should be available");

    let logout = use_callback(navigator, |_: (), navigator| {
        api::logout();
        navigator.push(&AppRoute::Login);
    });

    let profile_picture = format!(
        "/api/user/{}/picture#time={}",
        *profile_user_id,
        chrono::offset::Local::now().timestamp_millis()
    );

    html!(
        <>
            <CosmoTopBar
                profile_picture={profile_picture}
                has_right_item=true
                right_item_on_click={logout}
                right_item_label="Abmelden"
            >
                <CosmoTopBarItemLink<AppRoute> label="Rechtliches" to={AppRoute::LegalRoot} />
                <CosmoTopBarItemLink<AppRoute> label="Bambussupport" to={AppRoute::SupportRoot} />
            </CosmoTopBar>
        </>
    )
}

#[function_component(TopBarLegal)]
fn top_bar_legal() -> Html {
    log::debug!("Render top bar");
    let navigator = use_navigator().expect("Navigator should be available");

    let back = use_callback(navigator, |_: (), navigator| {
        navigator.push(&AppRoute::BambooGroveRoot);
    });

    html!(
        <CosmoTopBar
            profile_picture="/static/logo.webp"
            has_right_item=true
            right_item_on_click={back}
            right_item_label="Zum Hain"
        >
            <CosmoTopBarItemLink<AppRoute> label="" to={AppRoute::BambooGroveRoot} />
        </CosmoTopBar>
    )
}

#[function_component(Layout)]
fn layout() -> Html {
    log::info!("Run layout");
    use_groves();

    html!(<Switch<AppRoute> render={switch_layout} />)
}

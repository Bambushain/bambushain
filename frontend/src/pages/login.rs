use std::rc::Rc;

use bounce::helmet::Helmet;
use bounce::query::use_query_value;
use stylist::{css, GlobalStyle};
use stylist::yew::use_style;
use yew::prelude::*;
use yew_cosmo::prelude::*;
use yew_icons::Icon;
use yew_router::hooks::use_navigator;

use pandaparty_entities::prelude::*;

use crate::{api, storage};
use crate::routing::AppRoute;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let navigator = use_navigator().expect("Navigator should be available");

    let profile_query = use_query_value::<api::Profile>(().into());

    let username_state = use_state_eq(|| AttrValue::from(""));
    let password_state = use_state_eq(|| AttrValue::from(""));
    let error_message_state = use_state_eq(|| AttrValue::from("Melde dich an und komm in die Pandaparty"));

    let request_successful_state = use_state_eq(|| false);
    let error_state = use_state_eq(|| false);

    let on_username_update = use_callback(|value: AttrValue, state| state.set(value), username_state.clone());
    let on_password_update = use_callback(|value: AttrValue, state| state.set(value), password_state.clone());

    let login_submit = {
        let username_state = username_state.clone();
        let password_state = password_state.clone();
        let error_message_state = error_message_state.clone();

        let request_successful = request_successful_state.clone();
        let error_state = error_state.clone();

        Callback::from(move |_: ()| {
            let username_state = username_state.clone();
            let password_state = password_state.clone();
            let error_message_state = error_message_state.clone();

            let request_successful = request_successful.clone();
            let error_state = error_state.clone();

            let profile_query = profile_query.clone();

            let navigator = navigator.clone();

            yew::platform::spawn_local(async move {
                match api::login(Rc::new(Login { username: (*username_state).to_string(), password: (*password_state).to_string() })).await {
                    Ok(result) => {
                        request_successful.set(true);
                        error_state.set(false);
                        error_message_state.set(AttrValue::from(""));
                        storage::set_token(result.token);
                        let _ = profile_query.refresh().await;
                        navigator.push(&AppRoute::PandaPartyRoot);
                    }
                    Err(_) => {
                        request_successful.set(false);
                        error_state.set(true);
                        error_message_state.set("Die Zugangsdaten sind ungültig".into());
                    }
                }
            })
        })
    };

    let global_style = GlobalStyle::new(css!(r#"
body.panda-login {
    --black: #ffffff;
    --white: transparent;
    --primary-color: #9F2637;
    --control-border-color: var(--black);
    --negative-color: var(--black);

    --font-weight-bold: bold;
    --font-weight-normal: normal;
    --font-weight-light: 300;
    --font-family: Lato, sans-serif;
}

body.panda-login button {
    --control-border-color: var(--primary-color);
    --black: var(--primary-color);
}

body.panda-login input {
    --primary-color: var(--control-border-color);
}

body.panda-login button:hover {
    color: #ffffff !important;
}"#)).expect("Should be able to create global style");

    let login_around_style = use_style!(r#"
position: fixed;
left: 0;
right: 0;
top: 0;
bottom: 0;
display: flex;
justify-content: center;
align-items: center;
height: 100vh;
width: 100vw;
background: url("/static/background-login.webp");
background-size: cover;
background-position-y: bottom;

font-family: var(--font-family);
color: var(--black);
    "#);

    let login_container_style = use_style!(r#"
background: rgba(255, 255, 255, 0.25);
padding: 32px 64px;
backdrop-filter: blur(24px) saturate(90%);
box-sizing: border-box;
margin-top: -20px;
min-width: 570px;
"#);
    let login_message_style = use_style!(r#"
font-size: 24px;
color: #fff;
font-weight: var(--font-weight-light);
font-family: var(--font-family);
display: flex;
gap: 8px;
align-items: center;
    "#);

    html!(
        <>
            <Helmet>
                <title>{"Anmelden"}</title>
                <body class="panda-login" />
                <style>
                    {global_style.get_style_str()}
                </style>
            </Helmet>
            <div class={login_around_style}>
                <div class={login_container_style}>
                    <CosmoTitle title="Anmelden" />
                    <p class={login_message_style}>
                        if *error_state {
                            <Icon icon_id={IconId::LucideXOctagon} style="stroke: #290403;" />
                        } else {
                            <Icon icon_id={IconId::LucideLogIn} />
                        }
                        {(*error_message_state).clone()}
                    </p>
                    <CosmoForm on_submit={login_submit} buttons={html!(<CosmoButton label="Anmelden" is_submit={true} />)}>
                        <CosmoTextBox id="username" required={true} value={(*username_state).clone()} on_input={on_username_update} label="Benutzername" />
                        <CosmoTextBox id="password" input_type={CosmoTextBoxType::Password} required={true} value={(*password_state).clone()} on_input={on_password_update} label="Passwort" />
                    </CosmoForm>
                </div>
            </div>
        </>
    )
}

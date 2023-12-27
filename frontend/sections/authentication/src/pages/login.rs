use bamboo_frontend_base_routing::LegalRoute;
use stylist::yew::use_style;
use yew::prelude::*;
use yew_cosmo::prelude::*;
use yew_hooks::{use_async, use_bool_toggle};
use yew_icons::Icon;
use yew_router::hooks::use_navigator;

use bamboo_entities::prelude::{ForgotPassword, Login};
use bamboo_frontend_base_routing::AppRoute;
use bamboo_frontend_base_storage as storage;

use crate::api;

#[function_component(LoginContent)]
fn login_content() -> Html {
    let navigator = use_navigator().expect("Navigator should be available");

    let email_state = use_state_eq(|| AttrValue::from(""));
    let password_state = use_state_eq(|| AttrValue::from(""));
    let two_factor_code_state = use_state_eq(|| AttrValue::from(""));

    let two_factor_code_requested_toggle = use_bool_toggle(false);
    let forgot_password_toggle = use_bool_toggle(false);

    let login = {
        let email_state = email_state.clone();
        let password_state = password_state.clone();
        let two_factor_code_state = two_factor_code_state.clone();

        let two_factor_code_requested_toggle = two_factor_code_requested_toggle.clone();

        use_async(async move {
            let two_factor_code = if (*two_factor_code_state).is_empty() {
                None
            } else {
                Some((*two_factor_code_state).to_string())
            };

            match api::login(Login::new(
                (*email_state).to_string(),
                (*password_state).to_string(),
                two_factor_code,
            ))
            .await
            {
                Ok(either::Left(result)) => {
                    storage::set_token(result.token);
                    navigator.push(&AppRoute::BambooGroveRoot);
                    Ok(())
                }
                Ok(either::Right(_)) => {
                    two_factor_code_requested_toggle.set(true);
                    Ok(())
                }
                Err(_) => {
                    if *two_factor_code_requested_toggle {
                        Err("Der Zwei Faktor Code ist ungültig")
                    } else {
                        Err("Die Email und das Passwort passen nicht zusammen")
                    }
                }
            }
        })
    };
    let forgot_password = {
        let email_state = email_state.clone();

        use_async(async move {
            api::forgot_password(ForgotPassword {
                email: (*email_state).to_string(),
            })
            .await
        })
    };

    let on_email_update = use_callback(email_state.clone(), |value, state| state.set(value));
    let on_password_update = use_callback(password_state.clone(), |value, state| state.set(value));
    let on_two_factor_code_update =
        use_callback(two_factor_code_state.clone(), |value: AttrValue, state| {
            state.set(value)
        });
    let login_submit = use_callback(
        (
            forgot_password_toggle.clone(),
            login.clone(),
            forgot_password.clone(),
        ),
        |_, (forgot_password_toggle, login, forgot_password)| {
            if **forgot_password_toggle {
                forgot_password.run();
            } else {
                login.run();
            }
        },
    );
    let forgot_password_click = use_callback(
        forgot_password_toggle.clone(),
        |_, forgot_password_toggle| {
            forgot_password_toggle.toggle();
        },
    );

    let login_around_style = use_style!(
        r#"
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

--black: #ffffff;
--white: transparent;

input {
    --primary-color: var(--control-border-color);
}
    "#
    );

    let login_container_style = use_style!(
        r#"
background: rgba(255, 255, 255, 0.25);
padding: 2rem 4rem;
backdrop-filter: blur(24px) saturate(90%);
box-sizing: border-box;
margin-top: 1.25rem;
min-width: 35.625rem;
max-width: 40rem;
border-radius: var(--border-radius);
"#
    );
    let login_message_style = use_style!(
        r#"
font-size: 1.5rem;
color: #fff;
font-weight: var(--font-weight-light);
font-family: var(--font-family);
display: flex;
gap: 0.5rem;
align-items: center;
    "#
    );

    html!(
        <div class={login_around_style}>
            <div class={classes!(login_container_style, "login-page")}>
                <CosmoTitle title="Anmelden" />
                <p class={login_message_style}>
                    if *forgot_password_toggle {
                        {"Gib deine Emailadresse oder deinen Namen ein, wenn du im Hain bist, schicken wir allen Mods eine Email mit der Bitte dein Passwort zurückzusetzen"}
                    } else if forgot_password.error.is_some() {
                        {"Leider konnten wir die Mods nicht erreichen, bitte wende dich direkt an einen Mod wenn du kannst oder an "}<CosmoAnchor href="mailto:panda.helferlein@bambushain.app">{"panda.helferlein@bambushain.app"}</CosmoAnchor>
                    } else if forgot_password.data.is_some() {
                        {"Wir haben den Mods geschrieben, bitte warte bis sich jemand bei dir meldet"}
                    } else if let Some(error) = &login.error {
                        <Icon icon_id={IconId::LucideXOctagon} style="stroke: var(--negative-color);" /> {error}
                    } else {
                        <Icon icon_id={IconId::LucideLogIn} /> {"Melde dich an und betrete den Bambushain"}
                    }
                </p>
                if !*two_factor_code_requested_toggle && !*forgot_password_toggle {
                    <CosmoForm on_submit={login_submit} buttons={html!(
                        <>
                            <CosmoButton state={CosmoButtonType::Default} label="Passwort vergessen" on_click={forgot_password_click} />
                            <CosmoButton state={CosmoButtonType::Primary} label="Anmelden" is_submit={true} />
                        </>
                    )}>
                        <CosmoTextBox id="email" required={true} value={(*email_state).clone()} on_input={on_email_update} label="Email oder Name" />
                        <CosmoTextBox id="password" input_type={CosmoTextBoxType::Password} required={true} value={(*password_state).clone()} on_input={on_password_update} label="Passwort" />
                    </CosmoForm>
                } else if *forgot_password_toggle {
                    <CosmoForm on_submit={login_submit} buttons={html!(
                        <>
                            <CosmoButton state={CosmoButtonType::Default} label="Zurück" on_click={forgot_password_click} />
                            <CosmoButton state={CosmoButtonType::Primary} label="Abschicken" is_submit={true} />
                        </>
                    )}>
                        <CosmoTextBox id="email" required={true} value={(*email_state).clone()} on_input={on_email_update} label="Email oder Name" />
                    </CosmoForm>
                } else {
                    <CosmoForm on_submit={login_submit} buttons={html!(<CosmoButton state={CosmoButtonType::Primary} label="Anmelden" is_submit={true} />)}>
                        <CosmoTextBox required={true} readonly={true} id="email" value={(*email_state).clone()} on_input={on_email_update} label="Email" />
                        <CosmoTextBox required={true} readonly={true} id="password" input_type={CosmoTextBoxType::Password} value={(*password_state).clone()} on_input={on_password_update} label="Passwort" />
                        <CosmoTextBox required={true} id="twofactor" value={(*two_factor_code_state).clone()} on_input={on_two_factor_code_update} label="Zwei Faktor Code" />
                    </CosmoForm>
                }
                <div style="display: flex; gap: 1rem">
                    <CosmoAnchorLink<AppRoute> to={AppRoute::LegalRoot}>{"Impressum"}</CosmoAnchorLink<AppRoute>>
                    <CosmoAnchorLink<LegalRoute> to={LegalRoute::DataProtection}>{"Datenschutzerklärung"}</CosmoAnchorLink<LegalRoute>>
                </div>
            </div>
        </div>
    )
}

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    html!(
        <LoginContent />
    )
}

use bounce::helmet::Helmet;
use bounce::query::use_query_value;
use stylist::yew::use_style;
use yew::prelude::*;
use yew_cosmo::prelude::*;
use yew_icons::Icon;
use yew_router::hooks::use_navigator;

use bamboo_entities::prelude::{ForgotPassword, Login};
use bamboo_frontend_base_routing::AppRoute;
use bamboo_frontend_base_storage as storage;

use crate::api;
use crate::models;

#[function_component(LoginContent)]
fn login_content() -> Html {
    let navigator = use_navigator().expect("Navigator should be available");

    let profile_query = use_query_value::<models::Profile>(().into());

    let email_state = use_state_eq(|| AttrValue::from(""));
    let password_state = use_state_eq(|| AttrValue::from(""));
    let two_factor_code_state = use_state_eq(|| AttrValue::from(""));
    let error_message_state =
        use_state_eq(|| AttrValue::from("Melde dich an und betrete den Bambushain"));

    let two_factor_code_requested_state = use_state_eq(|| false);
    let error_state = use_state_eq(|| false);
    let forgot_password_state = use_state_eq(|| false);

    let on_email_update = use_callback(email_state.clone(), |value: AttrValue, state| {
        state.set(value)
    });
    let on_password_update = use_callback(password_state.clone(), |value: AttrValue, state| {
        state.set(value)
    });
    let on_two_factor_code_update =
        use_callback(two_factor_code_state.clone(), |value: AttrValue, state| {
            state.set(value)
        });

    let login_submit = {
        let email_state = email_state.clone();
        let password_state = password_state.clone();
        let two_factor_code_state = two_factor_code_state.clone();
        let error_message_state = error_message_state.clone();

        let two_factor_code_requested_state = two_factor_code_requested_state.clone();
        let error_state = error_state.clone();
        let forgot_password_state = forgot_password_state.clone();

        Callback::from(move |_: ()| {
            let email_state = email_state.clone();
            let password_state = password_state.clone();
            let error_message_state = error_message_state.clone();

            let two_factor_code_requested_state = two_factor_code_requested_state.clone();
            let error_state = error_state.clone();
            let forgot_password_state = forgot_password_state.clone();

            let profile_query = profile_query.clone();

            let navigator = navigator.clone();

            let two_factor_requested = *two_factor_code_requested_state;
            let two_factor_code = if two_factor_requested {
                Some((*two_factor_code_state).to_string())
            } else {
                None
            };

            yew::platform::spawn_local(async move {
                if *forgot_password_state {
                    if api::forgot_password(ForgotPassword {
                        email: (*email_state).to_string(),
                    })
                    .await
                    .is_ok()
                    {
                        error_message_state
                                .set("Wir haben den Mods geschrieben, bitte warte bis sich jemand bei dir meldet".into());
                        error_state.set(false);
                    } else {
                        error_message_state
                                .set("Leider konnten wir die Mods nicht erreichen, bitte wende dich direkt an einen Mod wenn du kannst oder an panda.helferlein@bambushain.app".into());
                        error_state.set(true);
                    }
                } else {
                    match api::login(Login::new(
                        (*email_state).to_string(),
                        (*password_state).to_string(),
                        two_factor_code,
                    ))
                    .await
                    {
                        Ok(either::Left(result)) => {
                            storage::set_token(result.token);
                            let _ = profile_query.refresh().await;
                            navigator.push(&AppRoute::BambooGroveRoot);
                            error_state.set(false);
                        }
                        Ok(either::Right(_)) => {
                            error_message_state
                                .set("Melde dich an und betrete den Bambushain".into());
                            two_factor_code_requested_state.set(true);
                            error_state.set(false);
                        }
                        Err(_) => {
                            if two_factor_requested {
                                error_message_state.set("Der Zwei Faktor Code ist ungültig".into());
                            } else {
                                error_message_state
                                    .set("Die Email und das Passwort passen nicht zusammen".into());
                            }
                            error_state.set(true);
                        }
                    }
                }
            })
        })
    };

    let forgot_password_click = use_callback(
        (forgot_password_state.clone(), error_message_state.clone()),
        |_, (forgot_password_state, error_message_state)| {
            forgot_password_state.set(true);
            error_message_state.set("Gib deine Emailadresse oder deinen Namen ein, wenn du im Hain bist, schicken wir allen Mods eine Email mit der Bitte dein Passwort zurückzusetzen".into())
        },
    );
    let forgot_password_back = use_callback(
        (forgot_password_state.clone(), error_message_state.clone()),
        |_, (forgot_password_state, error_message_state)| {
            forgot_password_state.set(false);
            error_message_state.set("Melde dich an und betrete den Bambushain".into())
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
                    if !*forgot_password_state {
                        if *error_state {
                            <Icon icon_id={IconId::LucideXOctagon} style="stroke: var(--negative-color);" />
                        } else {
                            <Icon icon_id={IconId::LucideLogIn} />
                        }
                    }
                    {(*error_message_state).clone()}
                </p>
                if !*two_factor_code_requested_state && !*forgot_password_state {
                    <CosmoForm on_submit={login_submit} buttons={html!(
                        <>
                            <CosmoButton state={CosmoButtonType::Default} label="Passwort vergessen" on_click={forgot_password_click} />
                            <CosmoButton state={CosmoButtonType::Primary} label="Anmelden" is_submit={true} />
                        </>
                    )}>
                        <CosmoTextBox id="email" required={true} value={(*email_state).clone()} on_input={on_email_update} label="Email oder Name" />
                        <CosmoTextBox id="password" input_type={CosmoTextBoxType::Password} required={true} value={(*password_state).clone()} on_input={on_password_update} label="Passwort" />
                    </CosmoForm>
                } else if *forgot_password_state {
                    <CosmoForm on_submit={login_submit} buttons={html!(
                        <>
                            <CosmoButton state={CosmoButtonType::Default} label="Zurück" on_click={forgot_password_back} />
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
            </div>
        </div>
    )
}

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    html!(
        <>
            <Helmet>
                <title>{"Anmelden"}</title>
            </Helmet>
            <LoginContent />
        </>
    )
}

use stylist::yew::use_style;
use yew::prelude::*;
use yew_cosmo::prelude::*;
use yew_hooks::{use_async, use_bool_toggle};
use yew_icons::Icon;
use yew_router::hooks::use_navigator;

use bamboo_common::core::entities::{ForgotPassword, Login};
use bamboo_frontend_pandas_base::routing::AppRoute;
use bamboo_frontend_pandas_base::storage;

use crate::{api, AuthLayout};

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let navigator = use_navigator().expect("Navigator should be available");

    let email_state = use_state_eq(|| AttrValue::from(""));
    let password_state = use_state_eq(|| AttrValue::from(""));
    let two_factor_code_state = use_state_eq(|| AttrValue::from(""));

    let two_factor_code_required_toggle = use_bool_toggle(false);
    let forgot_password_toggle = use_bool_toggle(false);

    let login = {
        let email_state = email_state.clone();
        let password_state = password_state.clone();
        let two_factor_code_state = two_factor_code_state.clone();

        let two_factor_code_required_toggle = two_factor_code_required_toggle.clone();

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
                    two_factor_code_required_toggle.set(true);
                    Ok(())
                }
                Err(_) => Err(if *two_factor_code_required_toggle {
                    "Der Zwei Faktor Code ist ungültig"
                } else {
                    "Die Email und das Passwort passen nicht zusammen"
                }),
            }
        })
    };
    let forgot_password = {
        let email_state = email_state.clone();

        let forgot_password_toggle = forgot_password_toggle.clone();

        use_async(async move {
            let res = api::forgot_password(ForgotPassword {
                email: (*email_state).to_string(),
            })
            .await;
            forgot_password_toggle.set(false);

            res
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

    let login_message_style = use_style!(
        r#"
font-size: 1.5rem;
font-weight: var(--font-weight-light);
font-family: var(--font-family);
display: flex;
gap: 0.5rem;
align-items: center;
    "#
    );

    html!(
        <AuthLayout title="Anmelden">
            <p class={login_message_style}>
                if *forgot_password_toggle {
                    { "Gib deine Emailadresse oder deinen Namen ein, wenn du in Bambushain registriert bist, schicken wir dir eine Email mit einem Link" }
                } else if forgot_password.error.is_some() {
                    { "Leider konnten wir dir die Email nicht schicken, bitte schreib direkt eine Email an " }
                    <CosmoAnchor href="mailto:panda.helferlein@bambushain.app">
                        { "panda.helferlein@bambushain.app" }
                    </CosmoAnchor>
                } else if forgot_password.data.is_some() {
                    { "Eine Email mit einem Link zum Zurücksetzen vom Passwort ist unterwegs" }
                } else if let Some(error) = &login.error {
                    <Icon icon_id={IconId::LucideXOctagon} style="stroke: var(--negative-color);" />
                    { error }
                } else {
                    <Icon icon_id={IconId::LucideLogIn} />
                    { "Melde dich an und betrete den Bambushain" }
                }
            </p>
            if !*two_factor_code_required_toggle && !*forgot_password_toggle {
                <CosmoForm
                    on_submit={login_submit}
                    buttons={html!(
                    <>
                        <CosmoButton state={CosmoButtonType::Information} label="Passwort vergessen" on_click={forgot_password_click} />
                        <CosmoButton state={CosmoButtonType::Primary} label="Anmelden" is_submit={true} />
                    </>
                )}
                >
                    <CosmoTextBox
                        id="email"
                        required=true
                        value={(*email_state).clone()}
                        on_input={on_email_update}
                        label="Email oder Name"
                    />
                    <CosmoTextBox
                        id="password"
                        input_type={CosmoTextBoxType::Password}
                        required=true
                        value={(*password_state).clone()}
                        on_input={on_password_update}
                        label="Passwort"
                    />
                </CosmoForm>
            } else if *forgot_password_toggle {
                <CosmoForm
                    on_submit={login_submit}
                    buttons={html!(
                    <>
                        <CosmoButton state={CosmoButtonType::Default} label="Zurück" on_click={forgot_password_click} />
                        <CosmoButton state={CosmoButtonType::Primary} label="Abschicken" is_submit={true} />
                    </>
                )}
                >
                    <CosmoTextBox
                        id="email"
                        required=true
                        value={(*email_state).clone()}
                        on_input={on_email_update}
                        label="Email oder Name"
                    />
                </CosmoForm>
            } else {
                <CosmoForm
                    on_submit={login_submit}
                    buttons={html!(<CosmoButton state={CosmoButtonType::Primary} label="Anmelden" is_submit={true} />)}
                >
                    <CosmoTextBox
                        required=true
                        readonly=true
                        id="email"
                        value={(*email_state).clone()}
                        on_input={on_email_update}
                        label="Email"
                    />
                    <CosmoTextBox
                        required=true
                        readonly=true
                        id="password"
                        input_type={CosmoTextBoxType::Password}
                        value={(*password_state).clone()}
                        on_input={on_password_update}
                        label="Passwort"
                    />
                    <CosmoTextBox
                        required=true
                        id="twofactor"
                        value={(*two_factor_code_state).clone()}
                        on_input={on_two_factor_code_update}
                        label="Zwei Faktor Code"
                    />
                </CosmoForm>
            }
        </AuthLayout>
    )
}

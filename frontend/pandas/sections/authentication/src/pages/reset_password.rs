use stylist::yew::use_style;
use yew::prelude::*;
use yew_cosmo::prelude::*;
use yew_hooks::{use_async, UseAsyncHandle};
use yew_router::hooks::{use_location, use_navigator};

use crate::{api, AuthLayout};
use bamboo_common::core::entities::ResetPassword;
use bamboo_common::frontend::api::ApiError;
use bamboo_frontend_pandas_base::routing::AppRoute;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ResetQuery {
    email: String,
    token: String,
}

#[function_component(ResetPasswordPage)]
pub fn reset_password_page() -> Html {
    let navigator = use_navigator().expect("Navigator should be available");
    let location = use_location().expect("Location should be available");

    let password_state = use_state_eq(|| AttrValue::from(""));
    let password_repeat_state = use_state_eq(|| AttrValue::from(""));

    let reset_password: UseAsyncHandle<(), ApiError> = {
        let password_state = password_state.clone();
        let location = location.clone();
        let navigator = navigator.clone();

        use_async(async move {
            let query = location
                .query::<ResetQuery>()
                .map_err(|_| ApiError::json_deserialize_error())?;

            api::reset_password(ResetPassword::new(
                query.email,
                query.token,
                (*password_state).to_string(),
            ))
            .await?;

            navigator.push(&AppRoute::Login);

            Ok(())
        })
    };

    let on_password_update = use_callback(password_state.clone(), |value, state| state.set(value));
    let on_password_repeat_update = use_callback(password_repeat_state.clone(), |value, state| {
        state.set(value)
    });
    let reset_password_submit = use_callback(reset_password.clone(), |_, reset_password| {
        reset_password.run();
    });

    let reset_message_style = use_style!(
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
        <AuthLayout title="Passwort zurücksetzen">
            <p class={reset_message_style}>
                if reset_password.error.is_some() {
                    {"Das hat leider nicht geklappt, der Link ist wahrscheinlich abgelaufen, bitte fordere einen neuen an"}
                } else {
                    {"Wenn du dein Passwort zurücksetzen willst, musst du unten einfach nur dein neues Passwort eingeben"}
                }
            </p>
            <CosmoForm
                on_submit={reset_password_submit}
                buttons={html!(
                <>
                    <CosmoButtonLink<AppRoute> state={CosmoButtonType::Information} label="Zur Anmeldung" to={AppRoute::Login} />
                    <CosmoButton state={CosmoButtonType::Primary} label="Passwort zurücksetzen" enabled={(*password_state).clone() == (*password_repeat_state).clone()} is_submit={true} />
                </>
            )}
            >
                <CosmoTextBox
                    id="password"
                    input_type={CosmoTextBoxType::Password}
                    required=true
                    value={(*password_state).clone()}
                    on_input={on_password_update}
                    label="Neues Passwort"
                />
                <CosmoTextBox
                    id="password"
                    input_type={CosmoTextBoxType::Password}
                    required=true
                    value={(*password_repeat_state).clone()}
                    on_input={on_password_repeat_update}
                    label="Neues Passwort wiederholen"
                />
            </CosmoForm>
        </AuthLayout>
    )
}

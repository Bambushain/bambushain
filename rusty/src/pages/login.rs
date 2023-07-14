use bounce::query::use_mutation;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::hooks::use_navigator;
use crate::api::my::Profile;
use crate::routing::AppRoute;
use crate::storage::get_token;

#[derive(PartialEq, Eq, Clone)]
struct State {
    username: AttrValue,
    password: AttrValue,
}

#[function_component(Login)]
pub fn login() -> Html {
    let state = use_state_eq(|| State {
        username: AttrValue::from(""),
        password: AttrValue::from(""),
    });
    let loading_state = use_state_eq(|| false);
    let error_state = use_state_eq(|| false);
    let submit = {
        let navigator = use_navigator();
        let login_user_state = use_mutation::<Profile>();
        let error_state = error_state.clone();
        let loading_state = loading_state.clone();
        let state = state.clone();
        Callback::from(move |evt: SubmitEvent| {
            evt.prevent_default();
            let username = state.username.to_string();
            let password = state.password.to_string();
            let navigator = navigator.clone();
            let login_user_state = login_user_state.clone();
            let error_state = error_state.clone();
            let loading_state = loading_state.clone();
            log::debug!("Spawn local async future");
            yew::platform::spawn_local(async move {
                log::debug!("Perform login");
                let login_user_state = login_user_state.clone();
                let loading_state = loading_state.clone();
                loading_state.set(true);
                match login_user_state.run(sheef_entities::Login {
                    username,
                    password,
                }).await {
                    Ok(_) => {
                        log::debug!("Redirect to {}, this should change at some point to the original requested uri", AppRoute::Sheef);
                        navigator.expect("Navigator should be there").push(&AppRoute::Sheef);
                        error_state.set(false);
                    }
                    Err(_) => error_state.set(true),
                };
                loading_state.set(false);
            });
        })
    };
    let update_username = {
        let state = state.clone();
        Callback::from(move |evt: InputEvent| {
            let password = state.password.clone();
            state.set(State {
                username: evt.target_unchecked_into::<HtmlInputElement>().value().into(),
                password,
            })
        })
    };
    let update_password = {
        let state = state.clone();
        Callback::from(move |evt: InputEvent| {
            let username = state.username.clone();
            state.set(State {
                username,
                password: evt.target_unchecked_into::<HtmlInputElement>().value().into(),
            })
        })
    };

    if get_token().is_none() {
        html!(
            <main class="container login" data-theme="light">
                <article class="grid">
                    <div>
                        <hgroup>
                            <h1>{"Anmelden"}</h1>
                            <h2>{"\"Das geht bestimmt sheef\", dafür stehen wir mit unserem Namen"}</h2>
                        </hgroup>
                        {if *error_state {
                            html!(<p data-msg="negative">{"Deine Anmeldedaten sind falsch"}</p>)
                        } else {
                            html!(<p data-msg="info">{"Gib deine Anmeldedaten ein"}</p>)
                        }}
                        <form onsubmit={submit}>
                            <input readonly={*loading_state} oninput={update_username} value={state.username.clone()} type="text" name="username" placeholder="Benutzername" aria-label="Benutzername" required=true />
                            <input readonly={*loading_state} oninput={update_password} value={state.password.clone()} type="password" name="password" placeholder="Passwort" aria-label="Passwort"
                                   autocomplete="current-password" required=true />
                            <button disabled={*loading_state} type="submit" class="contrast">{"Anmelden"}</button>
                        </form>
                    </div>
                    <div></div>
                </article>
            </main>
        )
    } else {
        html!(
            <Redirect<AppRoute> to={AppRoute::Sheef} />
        )
    }
}

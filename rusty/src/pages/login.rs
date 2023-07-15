use bounce::query::use_mutation;
use bounce::helmet::Helmet;
use bounce::use_atom_setter;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::hooks::use_navigator;
use crate::api::my::Profile;
use crate::routing::AppRoute;
use crate::storage::{CurrentUser, get_token};

#[derive(PartialEq, Eq, Clone)]
struct State {
    username: AttrValue,
    password: AttrValue,
}

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let state = use_state_eq(|| State {
        username: AttrValue::from(""),
        password: AttrValue::from(""),
    });
    let loading_state = use_state_eq(|| false);
    let error_state = use_state_eq(|| false);
    let profile_atom_setter = use_atom_setter::<CurrentUser>();
    let submit = {
        let navigator = use_navigator();
        let login_user_state = use_mutation::<Profile>();
        let error_state = error_state.clone();
        let loading_state = loading_state.clone();
        let profile_atom_setter = profile_atom_setter.clone();
        use_callback(move |evt: SubmitEvent, state| {
            evt.prevent_default();
            let username = state.username.to_string();
            let password = state.password.to_string();
            let navigator = navigator.clone();
            let login_user_state = login_user_state.clone();
            let error_state = error_state.clone();
            let loading_state = loading_state.clone();
            let profile_atom_setter = profile_atom_setter.clone();
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
                    Ok(profile) => {
                        log::debug!("Redirect to {}, this should change at some point to the original requested uri", AppRoute::Sheef);
                        profile_atom_setter(CurrentUser { profile: profile.user.clone() });
                        navigator.expect("Navigator should be there").push(&AppRoute::Sheef);
                        error_state.set(false);
                    }
                    Err(_) => error_state.set(true),
                };
                loading_state.set(false);
            });
        }, state.clone())
    };
    let update_username = {
        use_callback(move |evt: InputEvent, state| {
            let password = state.password.clone();
            state.set(State {
                username: evt.target_unchecked_into::<HtmlInputElement>().value().into(),
                password,
            })
        }, state.clone())
    };
    let update_password = {
        use_callback(move |evt: InputEvent, state| {
            let username = state.username.clone();
            state.set(State {
                username,
                password: evt.target_unchecked_into::<HtmlInputElement>().value().into(),
            })
        }, state.clone())
    };

    if get_token().is_none() {
        html!(
            <>
                <Helmet>
                    <title>{"Login"}</title>
                </Helmet>
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
            </>
        )
    } else {
        html!(
            <Redirect<AppRoute> to={AppRoute::Sheef} />
        )
    }
}

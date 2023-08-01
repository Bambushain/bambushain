use bounce::helmet::Helmet;
use bounce::query::use_mutation;
use bounce::use_atom_setter;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_feather::LogIn;
use yew_router::hooks::use_navigator;

use pandaparty_entities::prelude::*;

use crate::api::my::Profile;
use crate::routing::AppRoute;
use crate::storage::CurrentUser;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let loading_state = use_state_eq(|| false);
    let error_state = use_state_eq(|| false);
    let username_state = use_state_eq(|| AttrValue::from(""));
    let password_state = use_state_eq(|| AttrValue::from(""));

    let profile_atom_setter = use_atom_setter::<CurrentUser>();

    let submit = {
        let navigator = use_navigator();
        let login_user_state = use_mutation::<Profile>();
        let error_state = error_state.clone();
        let loading_state = loading_state.clone();
        let profile_atom_setter = profile_atom_setter.clone();

        use_callback(move |evt: SubmitEvent, (username_state, password_state)| {
            evt.prevent_default();
            let username = username_state.to_string();
            let password = password_state.to_string();
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
                match login_user_state.run(Login {
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
        }, (username_state.clone(), password_state.clone()))
    };
    let update_username = use_callback(|evt: InputEvent, state| state.set(evt.target_unchecked_into::<HtmlInputElement>().value().into()), username_state.clone());
    let update_password = use_callback(|evt: InputEvent, state| state.set(evt.target_unchecked_into::<HtmlInputElement>().value().into()), password_state.clone());

    html!(
        <>
            <Helmet>
                <title>{"Login"}</title>
            </Helmet>
            <main class="container login" data-theme="light">
                <article class="grid">
                    <div>
                        <h1>{"Anmelden"}</h1>
                        {if *error_state {
                            html!(<p data-msg="negative">{"Deine Anmeldedaten sind falsch"}</p>)
                        } else {
                            html!(<p data-msg="info">{"Gib deine Anmeldedaten ein"}</p>)
                        }}
                        <form onsubmit={submit}>
                            <input readonly={*loading_state} oninput={update_username} value={(*username_state).clone()} type="text" name="username" placeholder="Name" aria-label="Name" required=true />
                            <input readonly={*loading_state} oninput={update_password} value={(*password_state).clone()} type="password" name="password" placeholder="Passwort" aria-label="Passwort"
                                   autocomplete="current-password" required=true />
                            <button disabled={*loading_state} type="submit" class="contrast"><span class="small-gap-row"><LogIn color={"var(--color)"} />{"Anmelden"}</span></button>
                        </form>
                    </div>
                    <div></div>
                </article>
            </main>
        </>
    )
}

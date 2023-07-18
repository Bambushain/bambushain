use bounce::{use_atom_setter, use_atom_value};
use bounce::query::use_query_value;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_feather::{Key, LogOut, User};
use yew_router::prelude::*;

use sheef_entities::UpdateProfile;

use crate::api::{FORBIDDEN, NOT_FOUND};
use crate::api::authentication::logout;
use crate::api::my::{change_my_password, Profile, update_my_profile};
use crate::api::user::get_users;
use crate::pages::calendar::CalendarPage;
use crate::pages::crafter::CrafterPage;
use crate::pages::crew::CrewPage;
use crate::pages::fighter::FighterPage;
use crate::pages::kill::KillPage;
use crate::pages::mount::MountPage;
use crate::pages::savage_mount::SavageMountPage;
use crate::routing::{AppRoute, SheefRoute};
use crate::storage::CurrentUser;
use crate::ui::modal::PicoModal;

fn switch(route: SheefRoute) -> Html {
    match route {
        SheefRoute::Home => html!(<Redirect<SheefRoute> to={SheefRoute::Calendar} />),
        SheefRoute::Calendar => html!(<CalendarPage />),
        SheefRoute::Crew => html!(<CrewPage />),
        SheefRoute::Crafter => html!(<CrafterPage />),
        SheefRoute::Fighter => html!(<FighterPage />),
        SheefRoute::Mounts => html!(<MountPage />),
        SheefRoute::SavageMounts => html!(<SavageMountPage />),
        SheefRoute::Kills => html!(<KillPage />),
    }
}

#[derive(Properties, Clone, PartialEq)]
struct ChangePasswordDialogProps {
    on_close: Callback<()>,
    mods: Vec<AttrValue>,
}

#[function_component(ChangePasswordDialog)]
fn change_password_dialog(props: &ChangePasswordDialogProps) -> Html {
    log::debug!("Open dialog to change password");
    let navigator = use_navigator();

    let error_state = use_state_eq(|| false);
    let loading_state = use_state_eq(|| false);

    let error_message_state = use_state_eq(|| AttrValue::from(""));
    let old_password_state = use_state_eq(|| AttrValue::from(""));
    let new_password_state = use_state_eq(|| AttrValue::from(""));

    let update_old_password = use_callback(|evt: InputEvent, state| state.set(evt.target_unchecked_into::<HtmlInputElement>().value().into()), old_password_state.clone());
    let update_new_password = use_callback(|evt: InputEvent, state| state.set(evt.target_unchecked_into::<HtmlInputElement>().value().into()), new_password_state.clone());

    let on_close = props.on_close.clone();
    let on_save = {
        let error_state = error_state.clone();
        let loading_state = loading_state.clone();

        let error_message_state = error_message_state.clone();
        let old_password_state = old_password_state.clone();
        let new_password_state = new_password_state.clone();

        Callback::from(move |evt: SubmitEvent| {
            loading_state.set(true);

            log::debug!("Perform password change");
            evt.prevent_default();
            let navigator = navigator.clone();

            let error_state = error_state.clone();
            let loading_state = loading_state.clone();

            let error_message_state = error_message_state.clone();
            let old_password_state = old_password_state.clone();
            let new_password_state = new_password_state.clone();

            yew::platform::spawn_local(async move {
                error_state.set(match change_my_password((*old_password_state).to_string(), (*new_password_state).to_string()).await {
                    Ok(_) => {
                        log::debug!("Password change was successful, now logout");
                        logout();
                        navigator.expect("Navigator should be available").push(&AppRoute::Login);

                        false
                    }
                    Err(err) => match err.code {
                        FORBIDDEN => {
                            log::warn!("The old password is wrong");
                            error_message_state.set(AttrValue::from("Das alte Passwort ist falsch. Wenn du möchtest dass es von einem Mod zurückgesetzt wird, einfach anschreiben"));

                            true
                        }
                        NOT_FOUND => {
                            log::warn!("The user was not found");
                            error_message_state.set(AttrValue::from("Du wurdest scheinbar gelöscht, bitte versuch es erneut um einen Fehler auszuschließen"));

                            true
                        }
                        _ => {
                            log::warn!("Failed to change the password {err}");
                            error_message_state.set(AttrValue::from("Leider konnte dein Passwort nicht geändert werden, bitte wende dich an Azami"));

                            true
                        }
                    }
                });
                loading_state.set(false);
            });
        })
    };

    html!(
        <PicoModal title="Passwort ändern" on_close={on_close.clone()} open={true} buttons={html!(
            <>
                <button onclick={move |_| on_close.emit(())} type="button" class="secondary">{"Abbrechen"}</button>
                <button form="update-password-modal" aria-busy={(*loading_state).to_string()} type="submit">{"Passwort ändern"}</button>
            </>
        )}>
            {if *error_state {
                html!(
                    <p data-msg="negative">
                        {(*error_message_state).clone()}<br />
                        <strong>{"Mods"}</strong>
                        <ul>
                            {for props.mods.iter().map(|user| html!(
                                <li key={user.to_string()}>{user.clone()}</li>
                            ))}
                        </ul>
                    </p>
                )
            } else {
                html!(
                    <p data-msg="info">
                        {"Hier kannst du dein Passwort ändern, falls du dich an dein altes Passwort nicht erinnern kannst, wende dich an einen Mod"}<br />
                        <strong>{"Mods"}</strong>
                        <ul>
                            {for props.mods.iter().map(|user| html!(
                                <li key={user.to_string()}>{user.clone()}</li>
                            ))}
                        </ul>
                    </p>
                )
            }}
            <form id="update-password-modal" onsubmit={on_save.clone()}>
                <label for="old-password">{"Aktuelles Passwort"}</label>
                <input oninput={update_old_password} readonly={*loading_state} type="password" value={(*old_password_state).clone()} required={true} id="old-password" name="old-password" />
                <label for="new-password">{"Neues Passwort"}</label>
                <input oninput={update_new_password} readonly={*loading_state} type="password" value={(*new_password_state).clone()} required={true} id="new-password" name="new-password" />
            </form>
        </PicoModal>
    )
}

#[derive(Properties, Clone, PartialEq)]
struct UpdateMyProfileDialogProps {
    on_close: Callback<()>,
}

#[function_component(UpdateMyProfileDialog)]
fn update_my_profile_dialog(props: &UpdateMyProfileDialogProps) -> Html {
    log::debug!("Open dialog to update profile");
    let authentication_state_query = use_query_value::<Profile>(().into());

    let user_atom = use_atom_value::<CurrentUser>();

    let error_state = use_state_eq(|| false);
    let loading_state = use_state_eq(|| false);

    let error_message_state = use_state_eq(|| AttrValue::from(""));
    let job_state = use_state_eq(|| AttrValue::from(user_atom.profile.job.clone()));
    let gear_level_state = use_state_eq(|| AttrValue::from(user_atom.profile.gear_level.clone()));

    let update_job = use_callback(|evt: InputEvent, state| state.set(evt.target_unchecked_into::<HtmlInputElement>().value().into()), job_state.clone());
    let update_gear_level = use_callback(|evt: InputEvent, state| state.set(evt.target_unchecked_into::<HtmlInputElement>().value().into()), gear_level_state.clone());

    let on_close = props.on_close.clone();
    let on_save = {
        let authentication_state_query = authentication_state_query;

        let error_state = error_state.clone();
        let loading_state = loading_state.clone();

        let error_message_state = error_message_state.clone();
        let job_state = job_state.clone();
        let gear_level_state = gear_level_state.clone();

        let on_close = on_close.clone();

        Callback::from(move |evt: SubmitEvent| {
            log::debug!("Perform password change");
            evt.prevent_default();

            loading_state.set(true);

            let authentication_state_query = authentication_state_query.clone();

            let error_state = error_state.clone();
            let loading_state = loading_state.clone();

            let error_message_state = error_message_state.clone();
            let job_state = job_state.clone();
            let gear_level_state = gear_level_state.clone();

            let on_close = on_close.clone();

            yew::platform::spawn_local(async move {
                error_state.set(match update_my_profile(UpdateProfile { gear_level: (*gear_level_state).to_string(), job: (*job_state).to_string() }).await {
                    Ok(_) => {
                        log::debug!("Profile update successful");
                        let _ = authentication_state_query.refresh().await;

                        on_close.emit(());

                        false
                    }
                    Err(err) => match err.code {
                        NOT_FOUND => {
                            log::warn!("The user was not found");
                            error_message_state.set(AttrValue::from("Du wurdest scheinbar gelöscht, bitte versuch es erneut um einen Fehler auszuschließen"));

                            true
                        }
                        _ => {
                            log::warn!("Failed to update the profile {err}");
                            error_message_state.set(AttrValue::from("Dein Profil konnte leider nicht geändert werden, bitte wende dich an Azami"));

                            true
                        }
                    }
                });
                loading_state.set(false);
            });
        })
    };

    html!(
        <PicoModal title="Profil bearbeiten" on_close={on_close.clone()} open={true} buttons={html!(
            <>
                <button onclick={move |_| on_close.emit(())} type="button" class="secondary">{"Abbrechen"}</button>
                <button form="update-profile-modal" aria-busy={(*loading_state).to_string()} type="submit">{"Profil speichern"}</button>
            </>
        )}>
            {if *error_state {
                html!(
                    <p data-msg="negative">{(*error_message_state).clone()}</p>
                )
            } else {
                html!()
            }}
            <form id="update-profile-modal" onsubmit={on_save.clone()}>
                <label for="old-password" >{"Rolle/Klasse (optional)"}</label>
                <input oninput={update_job} readonly={*loading_state} type="text" value={(*job_state).clone()} id="job" name="job" / >
                <label for="new-password" >{"Gear Level (optional)"}</label>
                <input oninput={update_gear_level} readonly={* loading_state} type="text" value={(*gear_level_state).clone()} id="gear-level" name="gear-level" />
            </form>
        </PicoModal>
    )
}

#[function_component(SheefLayout)]
pub fn sheef_layout() -> Html {
    let authentication_state_query = use_query_value::<Profile>(().into());

    let navigator = use_navigator();

    let change_password_open_state = use_state_eq(|| false);
    let update_my_profile_open_state = use_state_eq(|| false);

    let mods_state = use_state_eq(|| vec![] as Vec<AttrValue>);

    let logout_click = use_callback(move |evt: MouseEvent, _| {
        evt.prevent_default();
        let navigator = navigator.clone();
        logout();
        navigator.expect("Navigator should be available").push(&AppRoute::Login);
    }, ());
    let change_password_click = {
        let change_password_open_state = change_password_open_state.clone();

        let mods_state = mods_state.clone();

        Callback::from(move |evt: MouseEvent| {
            evt.prevent_default();

            let change_password_open_state = change_password_open_state.clone();

            let mods_state = mods_state.clone();

            yew::platform::spawn_local(async move {
                if let Ok(users) = get_users().await {
                    let mods = users.into_iter().filter_map(|user| if user.is_mod { Some(AttrValue::from(user.username)) } else { None }).collect::<Vec<AttrValue>>();
                    mods_state.set(mods);
                }

                change_password_open_state.set(true);
            });
        })
    };
    let update_my_profile_click = use_callback(|evt: MouseEvent, update_my_profile_open_state| {
        evt.prevent_default();
        update_my_profile_open_state.set(true);
    }, update_my_profile_open_state.clone());

    let change_password_close = use_callback(|_, change_password_open_state| change_password_open_state.set(false), change_password_open_state.clone());

    let update_my_profile_close = use_callback(|_, update_my_profile_open_state| update_my_profile_open_state.set(false), update_my_profile_open_state.clone());

    let profile_atom_setter = use_atom_setter::<CurrentUser>();

    match authentication_state_query.result() {
        Some(query_result) => match query_result {
            Ok(profile) => {
                profile_atom_setter(CurrentUser { profile: profile.user.clone() });
                html!(
                    <>
                        <BrowserRouter>
                            <nav class="container-fluid">
                                <ul>
                                    <li><strong>{"Sheef"}</strong></li>
                                    <li><Link<SheefRoute> to ={SheefRoute::Calendar}>{"Kalender"}</Link<SheefRoute>></li>
                                    <li><Link<SheefRoute> to={SheefRoute::Crew}>{"Crew"}</Link<SheefRoute>></li>
                                    <li><Link<SheefRoute> to={SheefRoute::Crafter}>{"Crafter"}</Link<SheefRoute>></li>
                                    <li><Link<SheefRoute> to={SheefRoute::Fighter}>{"Kämpfer"}</Link<SheefRoute>></li>
                                    <li><Link<SheefRoute> to={SheefRoute::Mounts}>{"Mounts"}</Link<SheefRoute>></li>
                                    <li><Link<SheefRoute> to={SheefRoute::SavageMounts}>{"Savage Mounts"}</Link<SheefRoute>></li>
                                    <li><Link<SheefRoute> to={SheefRoute::Kills}>{"Kills"}</Link<SheefRoute>></li>
                                </ul>
                                <ul>
                                    <li role="list" dir="rtl">
                                        <a href="#" aria-haspopup="listbox">{format!("{}s Sheef", profile.user.username.clone())}</a>
                                        <ul role="listbox">
                                            <li>
                                                <a href="#" onclick={update_my_profile_click}>
                                                    <span class="small-gap-row">
                                                        <User color={"var(--dropdown-color)"} />{"Mein Profil"}
                                                    </span>
                                                </a>
                                            </li>
                                            <li>
                                                <a href="#" onclick={change_password_click}>
                                                    <span class="small-gap-row">
                                                        <Key color={"var(--dropdown-color)"} />{"Passwort ändern"}
                                                    </span>
                                                </a>
                                            </li>
                                            <li></li>
                                            <li>
                                                <a href="#" onclick={logout_click}>
                                                    <span class="small-gap-row">
                                                        <LogOut color={"var(--dropdown-color)"} />{"Abmelden"}
                                                    </span>
                                                </a>
                                            </li>
                                        </ul>
                                    </li>
                                </ul>
                            </nav>
                            <div class="container-fluid">
                                <Switch<SheefRoute> render={switch} />
                            </div>
                        </BrowserRouter>
                        {if *change_password_open_state {
                            html!(
                                <ChangePasswordDialog mods={(*mods_state).clone()} on_close={change_password_close} />
                            )
                        } else {
                            html!()
                        }}
                        {if *update_my_profile_open_state {
                            html!(
                                <UpdateMyProfileDialog on_close={update_my_profile_close} />
                            )
                        } else {
                            html!()
                        }}
                    </>
                )
            }
            Err(_) => {
                log::debug!("First render, so lets send the request to check if the token is valid and see");
                html!(
                    <Redirect<AppRoute> to={AppRoute::Login} />
                )
            }
        },
        None => html!()
    }
}

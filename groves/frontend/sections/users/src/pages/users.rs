use yew::prelude::*;
use yew::virtual_dom::Key;
use yew_autoprops::autoprops;
use yew_cosmo::prelude::*;
use yew_hooks::{use_async, use_mount};
use yew_icons::{Icon, IconId};

use bamboo_common::core::entities::GroveUser;

use crate::api;

#[autoprops]
#[function_component(UsersPage)]
pub fn users_page(grove_id: i32) -> Html {
    log::debug!("Render users overview");
    let user_to_reset_password_state = use_state_eq(|| None as Option<GroveUser>);
    let user_to_make_mod_state = use_state_eq(|| None as Option<GroveUser>);

    let users_state = use_async(async move { api::get_users(grove_id).await });
    let grove_state = use_async(async move { api::get_grove(grove_id).await });
    let reset_password_state = {
        let user_to_reset_password_state = user_to_reset_password_state.clone();

        let users_state = users_state.clone();

        use_async(async move {
            if let Some(user) = (*user_to_reset_password_state).clone() {
                user_to_reset_password_state.set(None);
                let result = api::reset_password(grove_id, user.id).await;
                if result.is_ok() {
                    users_state.run();
                }

                result
            } else {
                Ok(())
            }
        })
    };
    let make_user_mod_state = {
        let user_to_make_mod_state = user_to_make_mod_state.clone();

        let users_state = users_state.clone();

        use_async(async move {
            if let Some(user) = (*user_to_make_mod_state).clone() {
                user_to_make_mod_state.set(None);
                let result = api::make_user_mod(grove_id, user.id).await;
                if result.is_ok() {
                    users_state.run();
                }

                result
            } else {
                Ok(())
            }
        })
    };

    let close_reset_password_dialog =
        use_callback(user_to_reset_password_state.clone(), |_, state| {
            state.set(None);
        });
    let open_reset_password_dialog =
        use_callback(user_to_reset_password_state.clone(), |grove, state| {
            state.set(Some(grove));
        });
    let confirm_reset_password_dialog = use_callback(reset_password_state.clone(), |_, state| {
        state.run();
    });

    let close_make_user_mod_dialog = use_callback(user_to_make_mod_state.clone(), |_, state| {
        state.set(None);
    });
    let open_make_user_mod_dialog = use_callback(user_to_make_mod_state.clone(), |grove, state| {
        state.set(Some(grove));
    });
    let confirm_make_user_mod_dialog = use_callback(make_user_mod_state.clone(), |_, state| {
        state.run();
    });

    {
        let users_state = users_state.clone();
        let grove_state = grove_state.clone();

        use_mount(move || {
            users_state.run();
            grove_state.run();
        });
    }

    html!(
        <>
            if grove_state.error.is_some() {
                <CosmoTitle title="Benutzer" />
            } else if let Some(data) = grove_state.data.clone() {
                <CosmoTitle title={format!("Benutzer in {}", data.name)} />
            }
            if users_state.loading {
                <CosmoProgressRing />
            } else if users_state.error.is_some() {
                <CosmoMessage message_type={CosmoMessageType::Negative} header="Fehler beim Laden" message="Leider konnten die Benutzer nicht geladen werden"/>
            } else if let Some(data) = users_state.data.clone() {
                if reset_password_state.error.is_some() {
                    <CosmoMessage message_type={CosmoMessageType::Negative} header="Fehler beim Zurücksetzen" message="Leider konnte das Passwort nicht zurückgesetzt werden" />
                }
                <CosmoTable headers={vec![AttrValue::from("#"), AttrValue::from("Name"), AttrValue::from("Email"), AttrValue::from("Ist Mod"), AttrValue::from("Aktionen")]}>
                    {for data.iter().map(|user| {
                        let open_reset_password_dialog = open_reset_password_dialog.clone();
                        let open_make_user_mod_dialog = open_make_user_mod_dialog.clone();

                        let reset_password_user = user.clone();
                        let user_to_make_mod = user.clone();

                        CosmoTableRow::from_table_cells(vec![
                            CosmoTableCell::from_html(html!({user.id}), None),
                            CosmoTableCell::from_html(html!({user.display_name.clone()}), None),
                            CosmoTableCell::from_html(html!({user.email.clone()}), None),
                            CosmoTableCell::from_html(html!(
                                if user.is_mod {
                                    <Icon icon_id={IconId::LucideCheck} />
                                } else {
                                    <Icon icon_id={IconId::LucideX} />
                                }
                            ), None),
                            CosmoTableCell::from_html(html!(
                                <>
                                    <CosmoToolbarGroup>
                                        <CosmoButton label="Passwort zurücksetzen" enabled={user.is_mod} on_click={move |_| open_reset_password_dialog.emit(reset_password_user.clone())} />
                                        <CosmoButton label="Modrechte entziehen" enabled={user.is_mod} />
                                        <CosmoButton label="Zum Mod machen" enabled={!user.is_mod} on_click={move |_| open_make_user_mod_dialog.emit(user_to_make_mod.clone())} />
                                    </CosmoToolbarGroup>
                                </>
                            ), None),
                        ], Some(Key::from(user.id.to_string())))
                    })}
                </CosmoTable>
                if let Some(user) = (*user_to_reset_password_state).clone() {
                    <CosmoConfirm title="Passwort zurücksetzen" message={format!("Soll das Passwort von {} zurückgesetzt werden?", user.display_name.clone())} decline_label="Nicht zurücksetzen" confirm_label="Passwort zurücksetzen" confirm_type={CosmoModalType::Warning} on_confirm={confirm_reset_password_dialog.clone()} on_decline={close_reset_password_dialog.clone()} />
                }
                if let Some(user) = (*user_to_make_mod_state).clone() {
                    <CosmoConfirm title="Zum Mod ernennen" message={format!("Soll der Benutzer {} zum Mod gemacht werden?\nBitte beachte, dass Mods volle Kontrolle über einen Hain haben.", user.display_name.clone())} decline_label="Nicht zum mod ernennen" confirm_label="Zum Mod ernennen" confirm_type={CosmoModalType::Warning} on_confirm={confirm_make_user_mod_dialog.clone()} on_decline={close_make_user_mod_dialog.clone()} />
                }
            }
        </>
    )
}

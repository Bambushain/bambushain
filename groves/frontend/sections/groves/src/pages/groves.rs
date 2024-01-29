use yew::prelude::*;
use yew::virtual_dom::Key;
use yew_autoprops::autoprops;
use yew_cosmo::prelude::*;
use yew_hooks::{use_async, use_bool_toggle, use_mount};
use yew_icons::{Icon, IconId};

use bamboo_common::core::entities::Grove;
use bamboo_common::frontend::api::CONFLICT;

use crate::api;

#[autoprops]
#[function_component(CreateGroveDialog)]
fn create_grove_dialog(on_saved: &Callback<Grove>, on_close: &Callback<()>) -> Html {
    let name_state = use_state_eq(|| AttrValue::from(""));
    let mod_name_state = use_state_eq(|| AttrValue::from(""));
    let mod_email_state = use_state_eq(|| AttrValue::from(""));

    let save_state = {
        let name_state = name_state.clone();
        let mod_name_state = mod_name_state.clone();
        let mod_email_state = mod_email_state.clone();

        let on_saved = on_saved.clone();

        use_async(async move {
            let result = api::create_grove(
                (*name_state).to_string(),
                (*mod_name_state).to_string(),
                (*mod_email_state).to_string(),
            )
            .await;
            if let Ok(result) = result.clone() {
                on_saved.emit(result);
            }

            result
        })
    };

    let update_name = use_callback(name_state.clone(), |value, state| state.set(value));
    let update_mod_name = use_callback(mod_name_state.clone(), |value, state| state.set(value));
    let update_mod_email = use_callback(mod_email_state.clone(), |value, state| state.set(value));

    let on_save = use_callback(save_state.clone(), |_, save_state| save_state.run());

    html!(
        <CosmoModal title="Hain hinzufügen" is_form={true} on_form_submit={on_save} buttons={
            html!(
                <>
                    <CosmoButton on_click={on_close.clone()} label="Abbrechen" />
                    <CosmoButton is_submit={true} label="Hain hinzufügen" />
                </>
            )}>
            <>
                if let Some(err) = &save_state.error {
                    if err.code == CONFLICT {
                        <CosmoMessage message="Ein Hain mit diesem Namen existiert bereits" message_type={CosmoMessageType::Negative} />
                    } else {
                        <CosmoMessage message="Der Hain konnte leider nicht hinzugefügt werden" message_type={CosmoMessageType::Negative} />
                    }
                } else {
                    <CosmoMessage message_type={CosmoMessageType::Information} header="Hain hinzufügen" message="Nachdem der Hain erstellt wurde bekommt der Mod sein Passwort per Mail" />
                }
                <CosmoInputGroup>
                    <CosmoTextBox label="Hainname" value={(*name_state).clone()} on_input={update_name} required={true} />
                    <CosmoTextBox label="Mod Name" value={(*mod_name_state).clone()} on_input={update_mod_name} required={true} />
                    <CosmoTextBox label="Mod Email" input_type={CosmoTextBoxType::Email} value={(*mod_email_state).clone()} on_input={update_mod_email} required={true} />
                </CosmoInputGroup>
            </>
        </CosmoModal>
    )
}

#[autoprops]
#[function_component(GrovesPage)]
pub fn groves_page() -> Html {
    log::debug!("Render groves overview");
    let create_grove_open_toggle = use_bool_toggle(false);
    let grove_to_suspend_state = use_state_eq(|| None as Option<Grove>);
    let grove_to_resume_state = use_state_eq(|| None as Option<Grove>);
    let grove_to_delete_state = use_state_eq(|| None as Option<Grove>);

    let groves_state = use_async(async move { api::get_groves().await });
    let suspend_grove_state = {
        let grove_to_suspend_state = grove_to_suspend_state.clone();

        let groves_state = groves_state.clone();

        use_async(async move {
            if let Some(grove) = (*grove_to_suspend_state).clone() {
                grove_to_suspend_state.set(None);
                let result = api::suspend_grove(grove.id).await;
                if result.is_ok() {
                    groves_state.run();
                }

                result
            } else {
                Ok(())
            }
        })
    };
    let resume_grove_state = {
        let grove_to_resume_state = grove_to_resume_state.clone();

        let groves_state = groves_state.clone();

        use_async(async move {
            if let Some(grove) = (*grove_to_resume_state).clone() {
                grove_to_resume_state.set(None);
                let result = api::resume_grove(grove.id).await;
                if result.is_ok() {
                    groves_state.run();
                }

                result
            } else {
                Ok(())
            }
        })
    };
    let delete_grove_state = {
        let grove_to_delete_state = grove_to_delete_state.clone();

        let groves_state = groves_state.clone();

        use_async(async move {
            if let Some(grove) = (*grove_to_delete_state).clone() {
                grove_to_delete_state.set(None);
                let result = api::delete_grove(grove.id).await;
                if result.is_ok() {
                    groves_state.run();
                }

                result
            } else {
                Ok(())
            }
        })
    };

    let close_create_dialog = use_callback(create_grove_open_toggle.clone(), |_, toggle| {
        toggle.set(false)
    });
    let open_create_dialog = use_callback(create_grove_open_toggle.clone(), |_, toggle| {
        toggle.set(true)
    });
    let saved_create_dialog = use_callback(
        (create_grove_open_toggle.clone(), groves_state.clone()),
        |_, (toggle, state)| {
            toggle.set(false);
            state.run();
        },
    );

    let close_suspend_dialog = use_callback(grove_to_suspend_state.clone(), |_, state| {
        state.set(None);
    });
    let open_suspend_dialog = use_callback(grove_to_suspend_state.clone(), |grove, state| {
        state.set(Some(grove));
    });
    let confirm_suspend_dialog = use_callback(suspend_grove_state.clone(), |_, state| {
        state.run();
    });

    let close_resume_dialog = use_callback(grove_to_resume_state.clone(), |_, state| {
        state.set(None);
    });
    let open_resume_dialog = use_callback(grove_to_resume_state.clone(), |grove, state| {
        state.set(Some(grove));
    });
    let confirm_resume_dialog = use_callback(resume_grove_state.clone(), |_, state| {
        state.run();
    });

    let close_delete_dialog = use_callback(grove_to_delete_state.clone(), |_, state| {
        state.set(None);
    });
    let open_delete_dialog = use_callback(grove_to_delete_state.clone(), |grove, state| {
        state.set(Some(grove));
    });
    let confirm_delete_dialog = use_callback(delete_grove_state.clone(), |_, state| {
        state.run();
    });

    {
        let groves_state = groves_state.clone();

        use_mount(move || {
            groves_state.run();
        });
    }

    html!(
        <>
            <CosmoTitle title="Haine" />
            if groves_state.loading {
                <CosmoProgressRing />
            } else if groves_state.error.is_some() {
                <CosmoMessage message_type={CosmoMessageType::Negative} header="Fehler beim Laden" message="Leider konnten die Haine nicht geladen werden"/>
            } else if let Some(data) = groves_state.data.clone() {
                if suspend_grove_state.error.is_some() {
                    <CosmoMessage message_type={CosmoMessageType::Negative} header="Fehler beim Pausieren" message="Leider konnte der Hain nicht pausiert werden" />
                }
                if resume_grove_state.error.is_some() {
                    <CosmoMessage message_type={CosmoMessageType::Negative} header="Fehler beim Starten" message="Leider konnte der Hain nicht gestartet werden" />
                }
                if delete_grove_state.error.is_some() {
                    <CosmoMessage message_type={CosmoMessageType::Negative} header="Fehler beim Löschen" message="Leider konnte der Hain nicht gelöscht werden" />
                }
                <CosmoToolbar>
                    <CosmoToolbarGroup>
                        <CosmoButton label="Neuer Hain" on_click={open_create_dialog} />
                    </CosmoToolbarGroup>
                </CosmoToolbar>
                <CosmoTable headers={vec![AttrValue::from("#"), AttrValue::from("Name"), AttrValue::from("Pausiert"), AttrValue::from("Aktiviert"), AttrValue::from("Aktionen")]}>
                    {for data.iter().map(|grove| {
                        let open_suspend_dialog = open_suspend_dialog.clone();
                        let open_resume_dialog = open_resume_dialog.clone();
                        let open_delete_dialog = open_delete_dialog.clone();

                        let suspend_grove = grove.clone();
                        let resume_grove = grove.clone();
                        let delete_grove = grove.clone();

                        CosmoTableRow::from_table_cells(vec![
                            CosmoTableCell::from_html(html!({grove.id}), None),
                            CosmoTableCell::from_html(html!({grove.name.clone()}), None),
                            CosmoTableCell::from_html(html!(
                                if grove.is_suspended {
                                    <Icon icon_id={IconId::LucideCheck} />
                                } else {
                                    <Icon icon_id={IconId::LucideX} />
                                }
                            ), None),
                            CosmoTableCell::from_html(html!(
                                if grove.is_enabled {
                                    <Icon icon_id={IconId::LucideCheck} />
                                } else {
                                    <Icon icon_id={IconId::LucideX} />
                                }
                            ), None),
                            CosmoTableCell::from_html(html!(
                                <>
                                    <CosmoToolbarGroup>
                                        <CosmoButton label="Mods anzeigen" />
                                        <CosmoButton label="Starten" enabled={grove.is_suspended} on_click={move |_| open_resume_dialog.emit(resume_grove.clone())} />
                                        <CosmoButton label="Pausieren" enabled={!grove.is_suspended} on_click={move |_| open_suspend_dialog.emit(suspend_grove.clone())} />
                                        <CosmoButton label="Löschen" on_click={move |_| open_delete_dialog.emit(delete_grove.clone())} />
                                    </CosmoToolbarGroup>
                                </>
                            ), None),
                        ], Some(Key::from(grove.id.to_string())))
                    })}
                </CosmoTable>
                if *create_grove_open_toggle {
                    <CreateGroveDialog on_close={close_create_dialog} on_saved={saved_create_dialog} />
                }
                if let Some(grove) = (*grove_to_suspend_state).clone() {
                    <CosmoConfirm title="Hain pausieren" message={format!("Soll der Hain {} pausiert werden? Wenn der Hain pausiert wird kann sich niemand mehr anmelden.", grove.name.clone())} decline_label="Nicht pausieren" confirm_label="Hain pausieren" confirm_type={CosmoModalType::Warning} on_confirm={confirm_suspend_dialog.clone()} on_decline={close_suspend_dialog.clone()} />
                }
                if let Some(grove) = (*grove_to_resume_state).clone() {
                    <CosmoConfirm title="Hain starten" message={format!("Soll der Hain {} gestartet werden? Wenn der Hain gestartet wird können sich die Pandas wieder anmelden.", grove.name.clone())} decline_label="Nicht starten" confirm_label="Hain starten" confirm_type={CosmoModalType::Warning} on_confirm={confirm_resume_dialog.clone()} on_decline={close_resume_dialog.clone()} />
                }
                if let Some(grove) = (*grove_to_delete_state).clone() {
                    <CosmoConfirm title="Hain löschen" message={format!("Soll der Hain {} gelöscht werden? Wenn der Hain gelöscht wird, werden alle Pandas, Events und Charaktere.\nEine Alternative ist es den Hain zu pausieren und den Mods Bescheid zu geben warum du den Hain pausiert hast", grove.name.clone())} decline_label="Nicht löschen" confirm_label="Hain löschen" confirm_type={CosmoModalType::Negative} on_confirm={confirm_delete_dialog.clone()} on_decline={close_delete_dialog.clone()} />
                }
            }
        </>
    )
}

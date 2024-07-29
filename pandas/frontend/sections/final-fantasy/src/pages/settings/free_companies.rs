use stylist::yew::use_style;
use yew::prelude::*;
use yew_cosmo::prelude::*;
use yew_hooks::{use_async, use_bool_toggle, use_mount, use_unmount};
use yew_icons::Icon;

use bamboo_common::core::entities::*;
use bamboo_common::frontend::api::CONFLICT;
use bamboo_pandas_frontend_base::controls::{use_dialogs, BambooErrorMessage};

use crate::api;

#[function_component(FreeCompaniesPage)]
pub fn free_companies() -> Html {
    log::debug!("Render free companies page");
    log::debug!("Initialize state and callbacks");
    let add_open_state = use_bool_toggle(false);
    let edit_open_state = use_bool_toggle(false);

    let selected_id_ref = use_mut_ref(|| -1);

    let name_state = use_state_eq(|| AttrValue::from(""));

    let dialogs = use_dialogs();

    let free_companies_state = use_async(async move { api::get_free_companies().await });
    let create_state = {
        let name_state = name_state.clone();

        let add_open_state = add_open_state.clone();

        let free_companies_state = free_companies_state.clone();

        use_async(async move {
            api::create_free_company(FreeCompany::new((*name_state).to_string()))
                .await
                .map(|_| {
                    free_companies_state.run();
                    add_open_state.set(false);
                    name_state.set("".into())
                })
        })
    };
    let edit_state = {
        let name_state = name_state.clone();

        let selected_id_ref = selected_id_ref.clone();

        let edit_open_state = edit_open_state.clone();

        let free_companies_state = free_companies_state.clone();

        use_async(async move {
            api::update_free_company(
                *selected_id_ref.borrow(),
                FreeCompany::new((*name_state).to_string()),
            )
            .await
            .map(|_| {
                free_companies_state.run();
                edit_open_state.set(false);
                name_state.set("".into())
            })
        })
    };
    let delete_state = {
        let selected_id_ref = selected_id_ref.clone();

        let free_companies_state = free_companies_state.clone();

        use_async(async move {
            api::delete_free_company(*selected_id_ref.borrow())
                .await
                .map(|_| {
                    free_companies_state.run();
                })
        })
    };

    {
        let name_state = name_state.clone();

        use_unmount(move || {
            name_state.set("".into());
        })
    }
    {
        let free_companies_state = free_companies_state.clone();

        use_mount(move || {
            free_companies_state.run();
        })
    }

    let on_add_open = use_callback(
        (add_open_state.clone(), name_state.clone()),
        |_, (open_state, name_state)| {
            open_state.set(true);
            name_state.set("".into());
        },
    );
    let on_add_close = use_callback(add_open_state.clone(), |_, open_state| {
        open_state.set(false);
    });
    let on_add_save = use_callback(create_state.clone(), |_, state| state.run());
    let on_edit_open = use_callback(
        (
            selected_id_ref.clone(),
            name_state.clone(),
            edit_open_state.clone(),
        ),
        |(id, name): (i32, AttrValue), (selected_id_ref, name_state, open_state)| {
            *selected_id_ref.borrow_mut() = id;
            name_state.set(name);
            open_state.set(true);
        },
    );
    let on_edit_close = use_callback(edit_open_state.clone(), |_, open_state| {
        open_state.set(false);
    });
    let on_edit_save = use_callback(edit_state.clone(), |_, state| state.run());

    let on_delete = use_callback(delete_state.clone(), |_, state| state.run());
    let on_delete_open = use_callback(
        (
            selected_id_ref.clone(),
            on_delete.clone(),
            dialogs.clone(),
        ),
        |(id, name): (i32, String), (selected_id_ref, on_delete, dialogs)| {
            *selected_id_ref.borrow_mut() = id;

            dialogs.confirm(
                "Freie Gesellschaft löschen",
                format!("Soll die Freie Gesellschaft {name} wirklich gelöscht werden?"),
                "Freie Gesellschaft Löschen",
                "Nicht löschen",
                CosmoModalType::Warning,
                on_delete.clone(),
                Callback::noop(),
            )
        },
    );
    let update_name = use_callback(name_state.clone(), |val, state| state.set(val));

    let list_style = use_style!(
        r#"
display: flex;
flex-flow: row wrap;
gap: 0.125rem;
    "#
    );
    let item_style = use_style!(
        r#"
display: flex;
gap: 0.25rem;
flex: 0 0 100%;
min-width: 100%;
align-items: center;
    "#
    );

    html!(
        <>
            <CosmoTitle title="Freie Gesellschaften" />
            <CosmoToolbar>
                <CosmoToolbarGroup>
                    <CosmoButton label="Freie Gesellschaft hinzufügen" on_click={on_add_open} />
                </CosmoToolbarGroup>
            </CosmoToolbar>
            if free_companies_state.loading {
                <CosmoProgressRing />
            } else if let Some(data) = &free_companies_state.data {
                <div class={list_style}>
                    { for data.iter().map(|free_company| {
                        let delete_free_company = free_company.clone();
                        let edit_free_company = free_company.clone();

                        let on_delete_open = on_delete_open.clone();
                        let on_edit_open = on_edit_open.clone();

                        html!(
                            <div class={item_style.clone()}>
                                {free_company.name.clone()}
                                <Icon style="cursor: pointer;" width="1rem" height="1rem" icon_id={IconId::LucideEdit} onclick={move |_| on_edit_open.emit((edit_free_company.id, edit_free_company.name.clone().into()))} />
                                <Icon style="cursor: pointer;" width="1rem" height="1rem" icon_id={IconId::LucideTrash} onclick={move |_| on_delete_open.emit((delete_free_company.id, delete_free_company.name.clone().into()))} />
                            </div>
                        )
                    }) }
                </div>
            }
            if let Some(error) = free_companies_state.error.clone() {
                <BambooErrorMessage
                    message="Die Freien Gesellschaften konnten leider nicht geladen werden"
                    header="Fehler beim Laden"
                    page="free_companies"
                    form="free_companies"
                    error={error}
                />
            }
            if let Some(error) = delete_state.error.clone() {
                <BambooErrorMessage
                    message="Die Freien Gesellschaften konnten leider nicht geladen werden"
                    header="Fehler beim Laden"
                    page="free_companies"
                    form="free_companies"
                    error={error}
                />
            }
            if *edit_open_state {
                <CosmoModal
                    title="Freie Gesellschaft bearbeiten"
                    is_form=true
                    on_form_submit={on_edit_save}
                    buttons={html!(
                    <>
                        <CosmoButton on_click={on_edit_close} label="Abbrechen" />
                        <CosmoButton label="Freie Gesellschaft speichern" is_submit={true} />
                    </>
                )}
                >
                    if let Some(err) = edit_state.error.clone() {
                        if err.code == CONFLICT {
                            <CosmoMessage
                                message="Die Freie Gesellschaft existiert bereits"
                                message_type={CosmoMessageType::Negative}
                            />
                        } else {
                            <BambooErrorMessage
                                message="Die Freie Gesellschaften konnte leider nicht gespeichert werden"
                                header="Fehler beim Speichern"
                                page="free_companies"
                                form="edit_free_companies"
                                error={err}
                            />
                        }
                    }
                    <CosmoInputGroup>
                        <CosmoTextBox
                            label="Name"
                            on_input={update_name.clone()}
                            value={(*name_state).clone()}
                            required=true
                        />
                    </CosmoInputGroup>
                </CosmoModal>
            }
            if *add_open_state {
                <CosmoModal
                    title="Freie Gesellschaft hinzufügen"
                    is_form=true
                    on_form_submit={on_add_save}
                    buttons={html!(
                    <>
                        <CosmoButton on_click={on_add_close} label="Abbrechen" />
                        <CosmoButton label="Freie Gesellschaft hinzufügen" is_submit={true} />
                    </>
                )}
                >
                    if let Some(err) = create_state.error.clone() {
                        if err.code == CONFLICT {
                            <CosmoMessage
                                message="Die Freie Gesellschaft existiert bereits"
                                message_type={CosmoMessageType::Negative}
                            />
                        } else {
                            <BambooErrorMessage
                                message="Die Freie Gesellschaften konnte leider nicht hinzugefügt werden"
                                header="Fehler beim Speichern"
                                page="free_companies"
                                form="edit_free_companies"
                                error={err}
                            />
                        }
                    }
                    <CosmoInputGroup>
                        <CosmoTextBox
                            label="Name"
                            on_input={update_name.clone()}
                            value={(*name_state).clone()}
                            required=true
                        />
                    </CosmoInputGroup>
                </CosmoModal>
            }
        </>
    )
}

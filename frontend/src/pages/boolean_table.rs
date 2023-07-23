use std::collections::BTreeMap;

use bounce::use_atom_value;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_feather::{Edit3, Trash2};

use crate::api::boolean_table::{BooleanTable as ApiBooleanTable, CaseInsensitiveString};
use crate::storage::CurrentUser;
use crate::ui::modal::{PicoConfirm, PicoModal};

#[derive(PartialEq, Clone)]
pub struct ModifyEntryModalSaveData {
    pub new_name: String,
    pub old_name: String,
}

#[derive(PartialEq, Clone)]
pub struct ActivationParams {
    pub user: String,
    pub key: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct BooleanTableProps {
    pub table_data: ApiBooleanTable,
    pub on_deactivate_entry: Callback<ActivationParams>,
    pub on_activate_entry: Callback<ActivationParams>,
    pub add_label: AttrValue,
    pub has_error: bool,
    pub error_message: AttrValue,
    pub is_loading: bool,

    pub add_title: AttrValue,
    pub edit_title: AttrValue,
    pub on_add_save: Callback<ModifyEntryModalSaveData>,
    pub on_edit_save: Callback<ModifyEntryModalSaveData>,
    pub add_save_label: AttrValue,
    pub edit_save_label: AttrValue,
    pub modify_modal_state: EntryModalState,
    pub on_modify_modal_state_change: Callback<EntryModalState>,

    pub delete_title: AttrValue,
    pub delete_message: AttrValue,
    pub delete_confirm: AttrValue,
    pub on_delete_click: Callback<AttrValue>,
    pub on_delete_confirm: Callback<AttrValue>,
    pub on_delete_decline: Callback<()>,
    pub delete_entry_open: bool,
}

#[derive(Clone, PartialEq, Properties)]
struct TableEntryProps {
    table_key: AttrValue,
    user: AttrValue,
    data: BTreeMap<CaseInsensitiveString, Vec<CaseInsensitiveString>>,
    current_user_is_mod: bool,
    current_user_username: AttrValue,
    on_deactivate_entry: Callback<ActivationParams>,
    on_activate_entry: Callback<ActivationParams>,
    is_checked: bool,
}

#[derive(Clone, PartialEq, Properties)]
struct ModifyEntryModalProps {
    title: AttrValue,
    on_close: Callback<()>,
    on_save: Callback<ModifyEntryModalSaveData>,
    save_label: AttrValue,
    is_loading: bool,
    #[prop_or(AttrValue::from(""))]
    name: AttrValue,
    has_error: bool,
    error_message: AttrValue,
}

#[derive(Clone, PartialEq)]
pub enum EntryModalState {
    Add,
    Edit(AttrValue),
    Closed,
}

#[function_component(ModifyEntryModal)]
fn modify_entry_modal(props: &ModifyEntryModalProps) -> Html {
    let name_state = use_state_eq(|| props.name.clone());

    let on_close = props.on_close.clone();
    let props_on_save = props.on_save.clone();
    let on_save = use_callback(move |evt: SubmitEvent, (state, props)| {
        evt.prevent_default();
        props_on_save.emit(ModifyEntryModalSaveData {
            new_name: (*state).to_string(),
            old_name: props.name.to_string(),
        });
    }, (name_state.clone(), props.clone()));
    let update_name = use_callback(|evt: InputEvent, state| state.set(evt.target_unchecked_into::<HtmlInputElement>().value().into()), name_state.clone());

    html!(
        <PicoModal title={props.title.clone()} open={true} on_close={props.on_close.clone()} buttons={html!(
            <>
                <button onclick={move |_| on_close.emit(())} type="button" class="secondary">{"Abbrechen"}</button>
                <button form="create-entry-modal" aria-busy={props.is_loading.to_string()} type="submit">{props.save_label.clone()}</button>
            </>
        )}>
            if props.has_error {
                <p data-msg="negative">{props.error_message.clone()}</p>
            }
            <form onsubmit={on_save} id="create-entry-modal">
                <label for="name">{"Name"}</label>
                <input oninput={update_name} readonly={props.is_loading} type="text" value={(*name_state).clone()} required={true} id="name" name="name" />
            </form>
        </PicoModal>
    )
}

#[function_component(TableEntry)]
fn table_entry(props: &TableEntryProps) -> Html {
    let user = props.user.clone();
    let is_checked_state = use_state_eq(|| props.is_checked);
    let can_edit = use_state_eq(|| props.current_user_is_mod || user.clone() == props.current_user_username.clone());

    use_effect_with_deps(|(is_checked_state, props)| {
        is_checked_state.set(props.is_checked);
    }, (is_checked_state.clone(), props.clone()));

    let onclick = use_callback(|evt: MouseEvent, (on_activate_entry, on_deactivate_entry, user, key)| {
        let value = evt.target_unchecked_into::<HtmlInputElement>().checked();
        let data = ActivationParams {
            user: user.to_string(),
            key: key.to_string(),
        };
        if value {
            on_activate_entry.emit(data);
        } else {
            on_deactivate_entry.emit(data);
        }
    }, (props.on_activate_entry.clone(), props.on_deactivate_entry.clone(), props.user.clone(), props.table_key.clone()));

    if *can_edit {
        log::debug!("You can edit the entry {} for user {}", props.table_key.clone(), props.user.clone());
    } else {
        log::debug!("You can't edit the entry {} for user {}", props.table_key.clone(), props.user.clone());
    }

    html!(
        <td>
            <input disabled={!*can_edit} type="checkbox" checked={*is_checked_state} role="switch" onclick={onclick} />
        </td>
    )
}

#[function_component(BooleanTable)]
pub fn boolean_table(props: &BooleanTableProps) -> Html {
    let current_user = use_atom_value::<CurrentUser>();

    let delete_entry_state = use_state_eq(|| AttrValue::from(""));

    let open_add_modal = use_callback(|_, props| {
        props.on_modify_modal_state_change.emit(EntryModalState::Add);
    }, props.clone());
    let open_edit_modal = use_callback(|key: String, props| {
        props.on_modify_modal_state_change.emit(EntryModalState::Edit(AttrValue::from(key)));
    }, props.clone());
    let close_entry_modal = use_callback(|_, props| {
        props.on_modify_modal_state_change.emit(EntryModalState::Closed);
    }, props.clone());
    let open_delete_modal = use_callback(|name: AttrValue, (state, props)| {
        state.set(name.clone());
        props.on_delete_click.emit(name);
    }, (delete_entry_state.clone(), props.clone()));
    let open_delete_modal_confirm = use_callback(|_, (state, props)| {
        props.on_delete_confirm.emit((**state).clone());
    }, (delete_entry_state, props.clone()));

    html!(
        <>
            if current_user.profile.is_mod {
                <nav>
                    <ul>
                        <li>
                            <button onclick={open_add_modal} type="button">{props.add_label.clone()}</button>
                        </li>
                    </ul>
                </nav>
            }
            <figure>
                <table role="grid">
                    <thead>
                        <tr>
                            <th>{"Crewmitglied"}</th>
                            {for props.table_data.keys.clone().into_iter().map(|key| html!(
                                <th key={key.to_string()}>
                                    <div class="small-gap-row">
                                        <span>{key.clone()}</span>
                                        {if current_user.profile.is_mod {
                                            let open_edit_modal = open_edit_modal.clone();
                                            let key = key.clone();

                                            let open_delete_modal = open_delete_modal.clone();
                                            let delete_key = key.clone();

                                            html!(
                                                <>
                                                    <a aria-label={props.edit_title.clone()} onclick={move |_| open_edit_modal.emit(key.to_string())}><Edit3 color="var(--color)" /></a>
                                                    <a aria-label={props.delete_title.clone()} onclick={move |_| open_delete_modal.emit(AttrValue::from(delete_key.to_string()))}><Trash2 color="var(--color)" /></a>
                                                </>
                                            )
                                        } else {
                                            html!()
                                        }}
                                    </div>
                                </th>
                            ))}
                        </tr>
                    </thead>
                    <tbody>
                        {for props.table_data.users.iter().map(|user| {
                            html!(
                                <tr key={user.to_string()}>
                                    <th>{user}</th>
                                    {for props.table_data.keys.iter().map(|key| {
                                        html!(
                                            <TableEntry is_checked={props.table_data.data.clone().get(key).expect("Key should exist").contains(user)} current_user_username={AttrValue::from(current_user.profile.username.clone())} on_activate_entry={props.on_activate_entry.clone()} on_deactivate_entry={props.on_deactivate_entry.clone()} current_user_is_mod={current_user.profile.is_mod} key={key.to_string()} data={props.table_data.data.clone()} table_key={AttrValue::from(key.to_string())} user={AttrValue::from(user.to_string())} />
                                        )
                                    })}
                                </tr>
                            )
                        })}
                    </tbody>
                </table>
            </figure>
            {match props.modify_modal_state.clone() {
                EntryModalState::Add => html!(
                    <ModifyEntryModal on_close={close_entry_modal} title={props.add_title.clone()} save_label={props.add_save_label.clone()} on_save={props.on_add_save.clone()} has_error={props.has_error} error_message={props.error_message.clone()} is_loading={props.is_loading} />
                ),
                EntryModalState::Edit(key) => html!(
                    <ModifyEntryModal on_close={close_entry_modal} title={props.edit_title.clone()} save_label={props.edit_save_label.clone()} on_save={props.on_edit_save.clone()} has_error={props.has_error} error_message={props.error_message.clone()} is_loading={props.is_loading} name={key} />
                ),
                EntryModalState::Closed => html!(),
            }}
            {match props.delete_entry_open {
                true => html!(
                    <PicoConfirm open={true} on_decline={props.on_delete_decline.clone()} on_confirm={open_delete_modal_confirm} title={props.delete_title.clone()} message={props.delete_message.clone()} confirm_label={props.delete_confirm.clone()} />
                ),
                false => html!(),
            }}
        </>
    )
}

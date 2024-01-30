use yew::prelude::*;
use yew::virtual_dom::Key;
use yew_autoprops::autoprops;
use yew_cosmo::prelude::*;
use yew_hooks::{use_async, use_mount};
use yew_icons::{Icon, IconId};

use crate::api;

#[autoprops]
#[function_component(UsersPage)]
pub fn users_page(grove_id: i32) -> Html {
    log::debug!("Render users overview");
    let users_state = use_async(async move { api::get_users(grove_id).await });
    let grove_state = use_async(async move { api::get_grove(grove_id).await });

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
                <CosmoMessage message_type={CosmoMessageType::Negative} header="Fehler beim Laden" message="Leider konnten die Haine nicht geladen werden"/>
            } else if let Some(data) = users_state.data.clone() {
                <CosmoTable headers={vec![AttrValue::from("#"), AttrValue::from("Name"), AttrValue::from("Email"), AttrValue::from("Ist Mod"), AttrValue::from("Aktionen")]}>
                    {for data.iter().map(|user| {
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
                                        <CosmoButton label="Passwort zurücksetzen" />
                                        <CosmoButton label="Modrechte entziehen" enabled={user.is_mod} />
                                        <CosmoButton label="Zum Mod machen" enabled={!user.is_mod} />
                                    </CosmoToolbarGroup>
                                </>
                            ), None),
                        ], Some(Key::from(user.id.to_string())))
                    })}
                </CosmoTable>
            }
        </>
    )
}

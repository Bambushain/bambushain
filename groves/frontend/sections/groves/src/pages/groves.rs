use yew::prelude::*;
use yew::virtual_dom::Key;
use yew_autoprops::autoprops;
use yew_cosmo::prelude::*;
use yew_hooks::{use_async, use_mount};
use yew_icons::{Icon, IconId};

use crate::api;

#[autoprops]
#[function_component(GrovesPage)]
pub fn groves_page() -> Html {
    log::debug!("Render groves overview");
    let groves_state = use_async(async move { api::get_groves().await });

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
                <CosmoMessage header="Fehler beim Laden" message="Leider konnten die Haine nicht geladen werden"/>
            } else if let Some(data) = groves_state.data.clone() {
                <CosmoToolbar>
                    <CosmoToolbarGroup>
                        <CosmoButton label="Neuer Hain" />
                    </CosmoToolbarGroup>
                </CosmoToolbar>
                <CosmoTable headers={vec![AttrValue::from("#"), AttrValue::from("Name"), AttrValue::from("Pausiert"), AttrValue::from("Aktiviert"), AttrValue::from("Aktionen")]}>
                    {for data.iter().map(|grove| {
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
                                        <CosmoButton label="Starten" enabled={grove.is_suspended} />
                                        <CosmoButton label="Pausieren" enabled={!grove.is_suspended} />
                                        <CosmoButton label="Löschen" />
                                    </CosmoToolbarGroup>
                                </>
                            ), None),
                        ], Some(Key::from(grove.id.to_string())))
                    })}
                </CosmoTable>
            }
        </>
    )
}

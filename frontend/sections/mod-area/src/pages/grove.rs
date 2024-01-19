use yew::prelude::*;
use yew_cosmo::prelude::*;
use yew_hooks::{use_async, use_bool_toggle, use_mount};
use yew_router::prelude::*;

use bamboo_frontend_base_routing::AppRoute;

use crate::api;

#[function_component(GroveManagementPage)]
pub fn grove_management_page() -> Html {
    let navigator = use_navigator().expect("Navigator needs to be some");

    let delete_grove_open_toggle = use_bool_toggle(false);
    let disable_grove_open_toggle = use_bool_toggle(false);
    let enable_grove_open_toggle = use_bool_toggle(false);

    let grove_state = use_async(async { api::get_grove().await });
    let disable_grove_state = {
        let navigator = navigator.clone();

        use_async(async move {
            api::disable_grove()
                .await
                .map(|_| navigator.push(&AppRoute::Login))
                .map_err(|_| navigator.push(&AppRoute::Login))
        })
    };
    let enable_grove_state = {
        let navigator = navigator.clone();

        use_async(async move {
            api::enable_grove()
                .await
                .map(|_| navigator.push(&AppRoute::Login))
                .map_err(|_| navigator.push(&AppRoute::Login))
        })
    };
    let delete_grove_state = {
        let navigator = navigator.clone();

        use_async(async move {
            api::delete_grove()
                .await
                .map(|_| navigator.push(&AppRoute::Login))
                .map_err(|_| navigator.push(&AppRoute::Login))
        })
    };

    let open_delete_grove = use_callback(delete_grove_open_toggle.clone(), |_, toggle| {
        toggle.set(true)
    });
    let open_disable_grove = use_callback(disable_grove_open_toggle.clone(), |_, toggle| {
        toggle.set(true)
    });
    let open_enable_grove = use_callback(enable_grove_open_toggle.clone(), |_, toggle| {
        toggle.set(true)
    });

    let close_delete_grove = use_callback(delete_grove_open_toggle.clone(), |_, toggle| {
        toggle.set(false)
    });
    let close_disable_grove = use_callback(disable_grove_open_toggle.clone(), |_, toggle| {
        toggle.set(false)
    });
    let close_enable_grove = use_callback(enable_grove_open_toggle.clone(), |_, toggle| {
        toggle.set(false)
    });

    let disable_grove = use_callback(disable_grove_state.clone(), |_, state| state.run());
    let enable_grove = use_callback(enable_grove_state.clone(), |_, state| state.run());
    let delete_grove = use_callback(delete_grove_state.clone(), |_, state| state.run());

    {
        let grove_state = grove_state.clone();

        use_mount(move || {
            grove_state.run();
        });
    }

    if grove_state.loading {
        html!(
            <CosmoProgressRing />
        )
    } else if grove_state.error.is_some() {
        html!(
            <>
                <CosmoTitle title="Hainverwaltung" />
                <CosmoMessage header="Fehler beim Laden" message="Dein Hain konnte nicht geladen werden, bitte wende dich an den Bambussupport" message_type={CosmoMessageType::Negative} />
            </>
        )
    } else if let Some(grove) = &grove_state.data {
        html!(
            <>
                <CosmoTitle title="Hainverwaltung" subtitle={grove.name.clone()} />
                <CosmoMessage header="Willkommen in der Hainverwaltung" message="In der Hainverwaltung hast du die Möglichkeit deinen Hain zu löschen oder zu deaktivieren" message_type={CosmoMessageType::Information} />
                if grove.is_enabled {
                    <CosmoMessage header="Hain deaktivieren" message="Du hast die Möglichkeit den Hain zu deaktivieren. Sobald er deaktiviert ist können sich nur noch Mods anmelden und haben nur noch Zugriff auf die Mod Area. Den Hain zu deaktivieren ist eine gute Alternative dazu ihn direkt zu löschen." message_type={CosmoMessageType::Warning} actions={html!(
                        <CosmoButton label="Hain deaktivieren" on_click={open_disable_grove} />
                    )} />
                    if *disable_grove_open_toggle {
                        <CosmoConfirm confirm_type={CosmoModalType::Warning} on_confirm={disable_grove} on_decline={close_disable_grove} confirm_label="Hain deaktivieren" decline_label="Hain nicht deaktivieren" title="Hain deaktivieren" message="Bist du sicher, dass du den Hain deaktivieren möchtest? Anschließend ist es nur noch Mods möglich sich anzumelden." />
                    }
                } else {
                    <CosmoMessage header="Hain aktivieren" message="Du hast die Möglichkeit den Hain wieder zu aktivieren. Sobald er aktiviert ist können sich alle Benutzer anmelden und dein Hain ist wieder aktiv." message_type={CosmoMessageType::Warning} actions={html!(
                        <CosmoButton label="Hain aktivieren" on_click={open_enable_grove} />
                    )} />
                    if *enable_grove_open_toggle {
                        <CosmoConfirm confirm_type={CosmoModalType::Warning} on_confirm={enable_grove} on_decline={close_enable_grove} confirm_label="Hain aktivieren" decline_label="Hain nicht aktivieren" title="Hain aktivieren" message="Wenn du den Hain aktivierst haben wieder alle Benutzer die Möglichkeit sich anzumelden." />
                    }
                }
                <CosmoMessage header="Hain löschen" message="Du hast die Möglichkeit den Hain zu löschen. Sobald der Hain gelöscht ist werden alle Benutzer, Events und Charaktere gelöscht. Wir haben 14 Tage lang nach dem Löschen die Möglichkeit die Daten aus einem unserer Backups wiederherzustellen, danach sind die Daten verloren." message_type={CosmoMessageType::Negative} actions={html!(
                    <CosmoButton label="Hain löschen" on_click={open_delete_grove} />
                )} />
                if *delete_grove_open_toggle {
                    <CosmoConfirm confirm_type={CosmoModalType::Negative} on_confirm={delete_grove} on_decline={close_delete_grove} confirm_label="Hain löschen" decline_label="Hain nicht löschen" title="Hain löschen" message="Wenn du den Hain löscht werden alle Benutzer und ihre Daten unwiderruflich gelöscht.\nBitte überleg dir ob es reicht ihn zu deaktivieren. Falls du ihn wirklich löschen möchtest mächten wir dich darum bitten allen Benutzern Bescheid zu geben, damit sie ihre Daten sichern können." />
                }
            </>
        )
    } else {
        html!()
    }
}

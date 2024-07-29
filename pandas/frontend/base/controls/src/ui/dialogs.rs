use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_cosmo::prelude::*;
use yew_hooks::{use_counter, use_list};

#[derive(PartialEq, Clone)]
pub struct BambooDialogs {
    on_confirm: Callback<ConfirmData>,
    on_alert: Callback<AlertData>,
}

#[derive(PartialEq, Clone)]
enum DialogType {
    Confirm((i32, ConfirmData)),
    Alert((i32, AlertData)),
}

#[derive(PartialEq, Clone)]
struct ConfirmData {
    title: AttrValue,
    message: AttrValue,
    confirm_label: AttrValue,
    decline_label: AttrValue,
    confirm_type: CosmoModalType,
    on_confirm: Callback<()>,
    on_decline: Callback<()>,
}

#[derive(PartialEq, Clone)]
struct AlertData {
    title: AttrValue,
    message: AttrValue,
    close_label: AttrValue,
    alert_type: CosmoModalType,
    on_close: Callback<()>,
}

impl BambooDialogs {
    pub fn confirm(
        &self,
        title: impl Into<String>,
        message: impl Into<String>,
        confirm_label: impl Into<String>,
        decline_label: impl Into<String>,
        confirm_type: CosmoModalType,
        on_confirm: Callback<()>,
        on_decline: Callback<()>,
    ) {
        self.on_confirm.emit(ConfirmData {
            title: title.into().into(),
            message: message.into().into(),
            confirm_label: confirm_label.into().into(),
            decline_label: decline_label.into().into(),
            confirm_type,
            on_confirm,
            on_decline,
        })
    }

    pub fn alert(
        &self,
        title: impl Into<String>,
        message: impl Into<String>,
        close_label: impl Into<String>,
        alert_type: CosmoModalType,
        on_close: Callback<()>,
    ) {
        self.on_alert.emit(AlertData {
            title: title.into().into(),
            message: message.into().into(),
            close_label: close_label.into().into(),
            alert_type,
            on_close,
        })
    }
}

#[autoprops]
#[function_component(BambooDialogsProvider)]
pub fn dialogs(children: &Children) -> Html {
    let id_counter = use_counter(0);
    let dialogs = use_list(vec![]);

    let on_confirm = use_callback(
        (dialogs.clone(), id_counter.clone()),
        |data: ConfirmData, (dialogs, id_counter)| {
            id_counter.increase();
            dialogs.push(DialogType::Confirm((**id_counter, data)))
        },
    );
    let on_alert = use_callback(
        (dialogs.clone(), id_counter.clone()),
        |data: AlertData, (dialogs, id_counter)| {
            id_counter.increase();
            dialogs.push(DialogType::Alert((**id_counter, data)))
        },
    );

    let ctx = use_state(|| BambooDialogs {
        on_alert,
        on_confirm,
    });

    let on_handle = use_callback(
        dialogs.clone(),
        |(id, callback): (i32, Callback<()>), dialogs| {
            dialogs.retain(|t| {
                let i = match t {
                    DialogType::Confirm((i, _)) => i,
                    DialogType::Alert((i, _)) => i,
                };

                *i != id
            });
            callback.emit(())
        },
    );

    let dialogs_portal = create_portal(
        html!(
            { for dialogs.clone().current().iter().cloned().map(move |dialog| {
                    match dialog {
                        DialogType::Confirm((id,data)) => {
                            let on_confirm = on_handle.clone();
                            let on_decline = on_handle.clone();

                            html!(
                                <CosmoConfirm
                                    title={data.title.clone()}
                                    message={data.message.clone()}
                                    confirm_label={data.confirm_label.clone()}
                                    decline_label={data.decline_label.clone()}
                                    confirm_type={data.confirm_type.clone()}
                                    on_confirm={move |_| on_confirm.emit((id.clone(), data.on_confirm.clone()))}
                                    on_decline={move |_| on_decline.emit((id.clone(), data.on_decline.clone()))}
                                />
                            )
                        }
                        DialogType::Alert((id, data)) => {
                            let on_close = on_handle.clone();

                            html!(
                                <CosmoAlert
                                    title={data.title.clone()}
                                    message={data.message.clone()}
                                    close_label={data.close_label.clone()}
                                    alert_type={data.alert_type.clone()}
                                    on_close={move |_| on_close.emit((id.clone(), data.on_close.clone()))}
                                />
                            )
                        }
                    }
                }) }
        ),
        gloo_utils::document()
            .get_element_by_id("bamboo_dialog_host")
            .unwrap(),
    );

    html!(
        <>
            <ContextProvider<BambooDialogs> context={(*ctx).clone()}>
                { children }
            </ContextProvider<BambooDialogs>>
            { dialogs_portal }
        </>
    )
}

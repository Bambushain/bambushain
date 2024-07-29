use bamboo_common::frontend::api::ApiError;
use bamboo_pandas_frontend_base_error::report_unknown_error;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_cosmo::prelude::*;
use yew_hooks::use_bool_toggle;

#[autoprops]
#[function_component(BambooErrorMessage)]
pub fn error_message(
    message: &AttrValue,
    header: &AttrValue,
    page: &AttrValue,
    form: &AttrValue,
    error: &ApiError,
) -> Html {
    let error_reported_toggle = use_bool_toggle(false);

    let report_error = use_callback(
        (
            error_reported_toggle.clone(),
            page.clone(),
            form.clone(),
            error.bamboo_error.clone(),
        ),
        move |_, (error_reported_toggle, page, form, error)| {
            report_unknown_error(page.to_string(), form.to_string(), error.clone());
            error_reported_toggle.set(true);
        },
    );

    let actions = if *error_reported_toggle {
        None
    } else {
        Some(html!(<CosmoButton label="Fehler melden" on_click={report_error} />))
    };

    html!(
        <CosmoMessage
            message_type={CosmoMessageType::Negative}
            header={header}
            message={message}
            actions={actions}
        />
    )
}

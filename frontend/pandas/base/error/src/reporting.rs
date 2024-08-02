use bamboo_common::core::entities::GlitchTipErrorRequest;
use bamboo_common::core::error::BambooError;
use bamboo_common::frontend::api;

pub fn report_unknown_error(
    page: impl Into<String> + Clone,
    form: impl Into<String> + Clone,
    error: BambooError,
) {
    let page = page.clone().into();
    let form = form.clone().into();

    yew::platform::spawn_local(async move {
        let url = gloo_utils::window().location().href();
        let _ = api::post_no_content(
            "/api/glitchtip",
            &GlitchTipErrorRequest::new(page, form, url.unwrap(), error.clone()),
        )
        .await;
    })
}

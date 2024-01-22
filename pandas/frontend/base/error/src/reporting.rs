use bamboo_common::core::entities::GlitchTipErrorRequest;
use bamboo_pandas_frontend_base_api as api;

pub fn report_unknown_error(
    page: impl Into<String> + Clone,
    form: impl Into<String> + Clone,
    error: api::ApiError,
) {
    let page = page.clone().into();
    let form = form.clone().into();

    yew::platform::spawn_local(async move {
        let url = gloo_utils::window().location().href();
        let _ = api::post_no_content(
            "/api/glitchtip",
            &GlitchTipErrorRequest::new(page, form, url.unwrap(), error.bamboo_error.clone()),
        )
            .await;
    })
}

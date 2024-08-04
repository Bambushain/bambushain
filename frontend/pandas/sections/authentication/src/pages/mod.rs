use bamboo_frontend_pandas_base::routing::{AppRoute, LegalRoute};
pub use login::*;
pub use reset_password::*;
use stylist::yew::use_style;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_cosmo::prelude::{CosmoAnchorLink, CosmoTitle};

mod login;
mod reset_password;

#[autoprops]
#[function_component(AuthLayout)]
pub fn auth_layout(children: &Children, title: &AttrValue) -> Html {
    let around_style = use_style!(
        r#"
position: fixed;
left: 0;
right: 0;
top: 0;
bottom: 0;
display: flex;
justify-content: center;
align-items: center;
height: 100vh;
width: 100vw;
background: url("/pandas/static/background-login.webp") !important;
background-size: cover !important;
background-position-y: bottom !important;
font-family: var(--font-family);

--information-hue: 158.4 !important;
--information-saturation: 22.5% !important;
--information-lightness-base: 21.8% !important;
    "#
    );

    let container_style = use_style!(
        r#"
background: var(--primary-color-alpha-50);
padding: 2rem 4rem;
backdrop-filter: blur(24px) saturate(90%);
box-sizing: border-box;
margin-top: 1.25rem;
min-width: 35.625rem;
max-width: 40rem;
border-radius: 1rem;

color: #fff;

--a-color: #fff;
--control-border-color: var(--primary-color);

input {
    background: var(--primary-color-alpha-50);
    color: #fff;
    border-top: none;
    border-left: none;
    border-right: none;
}
"#
    );

    html!(
        <div class={classes!("is--light", around_style)}>
            <div class={container_style}>
                <CosmoTitle title={title} />
                { children }
                <div style="display: flex; gap: 1rem; margin-top: 2rem">
                    <CosmoAnchorLink<AppRoute> to={AppRoute::LegalRoot}>
                        { "Impressum" }
                    </CosmoAnchorLink<AppRoute>>
                    <CosmoAnchorLink<LegalRoute> to={LegalRoute::DataProtection}>
                        { "Datenschutzerklärung" }
                    </CosmoAnchorLink<LegalRoute>>
                </div>
            </div>
        </div>
    )
}

use stylist::yew::use_style;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_cosmo::prelude::*;

#[autoprops]
#[function_component(LoginContent)]
fn login_content(on_login: &Callback<()>) -> Html {
    let login_around_style = use_style!(
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
background: url("/static/background-login.webp");
background-size: cover;
background-position-y: bottom;

font-family: var(--font-family);
color: var(--black);

--black: #ffffff;
--white: transparent;

input {
    --primary-color: var(--control-border-color);
}
    "#
    );

    let login_container_style = use_style!(
        r#"
background: rgba(255, 255, 255, 0.25);
padding: 2rem 4rem;
backdrop-filter: blur(24px) saturate(90%);
box-sizing: border-box;
margin-top: 1.25rem;
max-width: 40rem;
border-radius: var(--border-radius);
"#
    );

    html!(
        <div class={login_around_style}>
            <div class={classes!(login_container_style, "login-page")}>
                <CosmoTitle title="Anmelden" />
                <CosmoButton state={CosmoButtonType::Primary} on_click={on_login} label="Anmelden mit Zitadel"/>
            </div>
        </div>
    )
}

#[autoprops]
#[function_component(LoginPage)]
pub fn login_page(on_login: &Callback<()>) -> Html {
    html!(
        <LoginContent on_login={on_login} />
    )
}

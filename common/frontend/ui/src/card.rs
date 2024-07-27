use stylist::yew::use_style;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_autoprops::autoprops;
use yew_cosmo::prelude::{CosmoHeader, CosmoHeaderLevel, CosmoToolbarGroup};

#[autoprops]
#[function_component(BambooCard)]
pub fn bamboo_card(
    children: &Children,
    title: AttrValue,
    #[prop_or_default] buttons: &Option<VNode>,
    #[prop_or_default] prepend: &Option<VNode>,
) -> Html {
    let card_style = use_style!(
        r#"
display: flex;
flex-flow: column;
background: var(--modal-backdrop);
backdrop-filter: var(--modal-container-backdrop-filter);

button {
    border-top-left-radius: 0 !important;
    border-top-right-radius: 0 !important;
    width: 100%;
}
"#
    );
    let card_content_style = use_style!(
        r#"
border-radius: var(--border-radius);
display: grid;
grid-template-columns: [prepend] auto [text] 1fr; 
border: var(--input-border-width) solid var(--control-border-color);
        "#
    );
    let card_content_with_buttons_style = use_style!(
        r#"
margin-bottom: calc(var(--input-border-width) * -1 * 2);
        "#
    );
    let card_content_text_style = use_style!(
        r#"
padding: 0.5rem 1rem;
height: 100%;
display: flex;
flex-flow: column;
gap: 0.25rem;
grid-column: text;

h5 {
    margin-top: 0;
}
    "#
    );
    let card_content_prepend = use_style!(
        r#"
grid-column: prepend;

img {
    object-fit: cover;
    border-top-left-radius: var(--border-radius);
    border-bottom-left-radius: var(--border-radius);
}
    "#
    );

    let card_content_classes = buttons.clone().map_or_else(
        || classes!(card_content_style.clone()),
        |_| classes!(card_content_style, card_content_with_buttons_style),
    );

    html!(
        <div class={card_style}>
            <div class={card_content_classes}>
                if let Some(prepend) = prepend {
                    <div class={card_content_prepend}>
                        {prepend.clone()}
                    </div>
                }
                <div class={card_content_text_style}>
                    <CosmoHeader level={CosmoHeaderLevel::H5} header={title} />
                    {for children.iter()}
                </div>
            </div>
            if let Some(buttons) = buttons {
                <CosmoToolbarGroup>
                    {buttons.clone()}
                </CosmoToolbarGroup>
            }
        </div>
    )
}

#[autoprops]
#[function_component(BambooCardList)]
pub fn bamboo_card_list(children: &Children) -> Html {
    let class_list_style = use_style!(
        r#"
display: flex;
margin-top: 2rem;
flex-flow: row wrap;
gap: 1rem;
justify-content: flex-start;
"#
    );

    html!(
        <div class={class_list_style}>
            {for children.iter()}
        </div>
    )
}

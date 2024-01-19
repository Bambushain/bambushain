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
border: var(--input-border-width) solid var(--control-border-color);
border-radius: var(--border-radius);
padding: 0.5rem 1rem;
height: 100%;
display: flex;
flex-flow: column;
gap: 0.25rem;

h5 {
    margin-top: 0;
}
        "#
    );
    let card_content_with_buttons_style = use_style!(
        r#"
margin-bottom: calc(var(--input-border-width) * -1 * 2);
        "#
    );
    let card_content_classes = if buttons.is_some() {
        classes!(card_content_style, card_content_with_buttons_style)
    } else {
        classes!(card_content_style)
    };

    html!(
        <div class={card_style}>
            <div class={card_content_classes}>
                <CosmoHeader level={CosmoHeaderLevel::H5} header={title} />
                {for children.iter()}
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

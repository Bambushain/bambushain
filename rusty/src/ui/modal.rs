use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ModalProps {
    #[prop_or_default]
    pub children: Children,
    pub title: AttrValue,
    #[prop_or(false)]
    pub open: bool,
    pub on_close: Callback<()>,
}

#[function_component(PicoModal)]
pub fn modal(props: &ModalProps) -> Html {
    let modal_host = gloo::utils::document()
        .get_element_by_id("modal-container")
        .expect("Expected to find a #modal-container element");

    let close_click = use_callback(|evt: MouseEvent, props| {
        evt.prevent_default();
        props.on_close.emit(());
    }, props.clone());

    create_portal(
        html!(
            <dialog open={props.open}>
                <article>
                <header>
                    <a onclick={close_click} aria-label="Close" class="close"></a>
                    <strong>{props.title.clone()}</strong>
                </header>
                    {for props.children.iter()}
                </article>
            </dialog>
        ),
        modal_host,
    )
}
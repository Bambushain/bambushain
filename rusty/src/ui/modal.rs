use yew::prelude::*;
use yew::virtual_dom::VNode;

#[derive(Properties, PartialEq, Clone)]
pub struct ModalProps {
    #[prop_or_default]
    pub children: Children,
    pub buttons: VNode,
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
                    <footer class="gap-row-right">
                        {props.buttons.clone()}
                    </footer>
                </article>
            </dialog>
        ),
        modal_host,
    )
}

#[derive(Properties, PartialEq, Clone)]
pub struct ConfirmProps {
    pub title: AttrValue,
    pub message: AttrValue,
    #[prop_or(false)]
    pub open: bool,
    #[prop_or(AttrValue::from("Ok"))]
    pub confirm_label: AttrValue,
    #[prop_or(AttrValue::from("Abbrechen"))]
    pub decline_label: AttrValue,
    pub on_confirm: Callback<()>,
    pub on_decline: Callback<()>,
}

#[function_component(PicoConfirm)]
pub fn confirm(props: &ConfirmProps) -> Html {
    let on_decline = props.on_decline.clone();
    let on_confirm = props.on_confirm.clone();

    html!(
        <dialog open={props.open}>
            <article>
                <h3>{props.title.clone()}</h3>
                <p>{props.message.clone()}</p>
                <footer class="gap-row-right">
                    <button type="button" class="secondary" onclick={move |_| on_decline.emit(())}>{props.decline_label.clone()}</button>
                    <button type="button" onclick={move |_| on_confirm.emit(())}>{props.confirm_label.clone()}</button>
                </footer>
            </article>
        </dialog>
    )
}

#[derive(Properties, PartialEq, Clone)]
pub struct AlertProps {
    pub title: AttrValue,
    pub message: AttrValue,
    #[prop_or(false)]
    pub open: bool,
    #[prop_or(AttrValue::from("Alles klar"))]
    pub close_label: AttrValue,
    pub on_close: Callback<()>,
}

#[function_component(PicoAlert)]
pub fn alert(props: &AlertProps) -> Html {
    let on_close = props.on_close.clone();

    html!(
        <dialog open={props.open}>
            <article>
                <h3>{props.title.clone()}</h3>
                <p>{props.message.clone()}</p>
                <footer class="gap-row-right">
                    <button type="button" onclick={move |_| on_close.emit(())}>{props.close_label.clone()}</button>
                </footer>
            </article>
        </dialog>
    )
}

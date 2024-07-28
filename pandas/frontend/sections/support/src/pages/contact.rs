use stylist::yew::use_style;
use yew::prelude::*;
use yew_cosmo::prelude::*;
use yew_hooks::{use_async, use_bool_toggle};

use bamboo_common::core::entities::SupportRequest;

use crate::api;

#[derive(Clone, PartialEq, Properties)]
struct SupportSectionProps {
    pub children: Children,
    pub header: AttrValue,
    pub submit_label: AttrValue,
}

#[function_component(SupportSection)]
fn support_section(props: &SupportSectionProps) -> Html {
    let container_style = use_style!(
        r#"
max-width: calc(var(--input-width-large) + 10rem + var(--input-group-gap));
padding-top: 2rem;
display: flex;
flex-flow: column;

@media screen and (max-width: 1600px) {
    max-width: 100%;
}
    "#
    );

    let subject_state = use_state_eq(|| AttrValue::from(""));
    let message_state = use_state_eq(|| AttrValue::from(""));

    let send_state = use_bool_toggle(false);

    let update_subject = use_callback(subject_state.clone(), |value, state| state.set(value));
    let update_message = use_callback(message_state.clone(), |value, state| state.set(value));
    let close_alert = use_callback(send_state.clone(), |_, state| state.set(false));

    let request_state = {
        let subject_state = subject_state.clone();
        let message_state = message_state.clone();

        let send_state = send_state.clone();

        use_async(async move {
            let request = SupportRequest {
                subject: (*subject_state).to_string(),
                message: (*message_state).to_string(),
            };
            api::send_support_request(request)
                .await
                .map(|_| {
                    subject_state.set("".into());
                    message_state.set("".into());
                    send_state.set(true)
                })
                .map(|_| send_state.set(true))
        })
    };

    let send_request = use_callback(request_state.clone(), |_, state| state.run());

    html!(
        <>
            <CosmoTitle title={props.header.clone()} />
            <div class={container_style}>
                { props.children.clone() }
                <CosmoForm
                    on_submit={send_request}
                    buttons={html!(<CosmoButton state={CosmoButtonType::Primary} label={props.submit_label.clone()} is_submit={true} />)}
                >
                    <CosmoTextBox
                        width={CosmoInputWidth::Large}
                        required=true
                        value={(*subject_state).clone()}
                        on_input={update_subject}
                        label="Betreff"
                    />
                    <CosmoTextArea
                        width={CosmoInputWidth::Large}
                        rows=20
                        required=true
                        value={(*message_state).clone()}
                        on_input={update_message}
                        label="Nachricht"
                    />
                </CosmoForm>
            </div>
            if *send_state && request_state.error.is_some() {
                <CosmoAlert
                    on_close={close_alert.clone()}
                    alert_type={CosmoAlertType::Negative}
                    close_label="Alles klar"
                    title="Fehler beim Senden"
                    message="Leider konnte deine Nachricht nicht gesendet werden, bitte schick uns eine Email and panda.helferlein@bambushain.app"
                />
            } else if *send_state {
                <CosmoAlert
                    on_close={close_alert.clone()}
                    alert_type={CosmoAlertType::Positive}
                    close_label="Alles klar"
                    title="Abgesendet"
                    message="Deine Nachricht wurde abgeschickt, wir werden uns so schnell wie möglich bei dir melden 🐼"
                />
            }
        </>
    )
}

#[function_component(ContactPage)]
pub fn contact_page() -> Html {
    html!(
        <CosmoSideList has_add_button=false>
            <CosmoSideListItem label="Ich habe einen Fehler gefunden">
                <SupportSection header="Melde uns einen Fehler" submit_label="Fehler melden">
                    <CosmoMessage
                        message_type={CosmoMessageType::Information}
                        message="Du hast einen Fehler gefunden? Kein Problem, schreib bitte genau auf wie wir diesen Fehler nachstellen können und wir kümmern uns um einen Fix. Du bekommst eine Email mit Infos zum Status"
                    />
                </SupportSection>
            </CosmoSideListItem>
            <CosmoSideListItem label="Ich habe eine Frage">
                <SupportSection header="Frag uns was" submit_label="Frage stellen">
                    <CosmoMessage
                        message_type={CosmoMessageType::Information}
                        message="Du hast eine Frage an uns? Kein Problem, schreib einfach was du von uns wissen willst, wir werden unser Bestes geben deine Frage zu beantworten. Die Antwort bekommst du an die Emailadresse die in deinem Account eingerichtet ist"
                    />
                </SupportSection>
            </CosmoSideListItem>
            <CosmoSideListItem label="Hallo 👋">
                <SupportSection header="Hallo auch an dich 🐼" submit_label="Nachricht senden">
                    <CosmoMessage
                        message_type={CosmoMessageType::Information}
                        message="Du willst einfach mit uns reden und Hallo sagen? Dann schreib uns einfach deine Nachricht wir freuen uns immer von den Pandas im Bambushain zu hören"
                    />
                </SupportSection>
            </CosmoSideListItem>
        </CosmoSideList>
    )
}

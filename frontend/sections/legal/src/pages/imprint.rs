use stylist::yew::use_style;
use yew::prelude::*;
use yew_cosmo::prelude::*;

#[function_component(ImprintPage)]
pub fn imprint() -> Html {
    let address_style = use_style!(
        r#"
font-style: normal;
    "#
    );

    html!(
        <>
            <CosmoTitle title="Impressum" />
            <CosmoHeader header="Angaben gemäß § 5 TMG" level={CosmoHeaderLevel::H1} />
            <address class={address_style}>
                {"Imanuel Ulbricht"}<br />
                {"Dingworthstr. 15"}<br />
                {"31137 Hildesheim"}
            </address>
            <CosmoHeader header="Kontakt" level={CosmoHeaderLevel::H2} />
            <CosmoParagraph>
                {"E-Mail: "}<CosmoAnchor href="mailto:panda.helferlein@bambushain.app">{"panda.helferlein@bambushain.app"}</CosmoAnchor>
            </CosmoParagraph>
            <CosmoHeader header="Haftung für Inhalte" level={CosmoHeaderLevel::H2} />
            <CosmoParagraph>
                {"Als Diensteanbieter sind wir gemäß § 7 Abs.1 TMG für eigene Inhalte auf diesen Seiten nach den allgemeinen Gesetzen verantwortlich. Nach §§ 8 bis 10 TMG sind wir als Diensteanbieter jedoch nicht verpflichtet, übermittelte oder gespeicherte fremde Informationen zu überwachen oder nach Umständen zu forschen, die auf eine rechtswidrige Tätigkeit hinweisen."}
            </CosmoParagraph>
            <CosmoParagraph>
                {"Verpflichtungen zur Entfernung oder Sperrung der Nutzung von Informationen nach den allgemeinen Gesetzen bleiben hiervon unberührt. Eine diesbezügliche Haftung ist jedoch erst ab dem Zeitpunkt der Kenntnis einer konkreten Rechtsverletzung möglich. Bei Bekanntwerden von entsprechenden Rechtsverletzungen werden wir diese Inhalte umgehend entfernen."}
            </CosmoParagraph>
            <CosmoParagraph>{"Quelle: e-recht24.de"}</CosmoParagraph>
        </>
    )
}

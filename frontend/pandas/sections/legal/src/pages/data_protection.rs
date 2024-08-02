use stylist::yew::use_style;
use yew::prelude::*;
use yew_cosmo::prelude::*;

#[function_component(DataProtectionPage)]
pub fn data_protection() -> Html {
    let address_style = use_style!(
        r#"
font-style: normal;
    "#
    );

    html!(
        <>
            <CosmoTitle title="Datenschutzerklärung" />
            <CosmoParagraph>
                { "Mit dieser Datenschutzerklärung möchten wir Sie über Art, Umfang und Zweck der Verarbeitung von personenbezogenen Daten (im Folgenden auch nur als \"Daten\" bezeichnet) aufklären. Personenbezogene Daten sind alle Daten, die einen persönlichen Bezug zu Ihnen aufweisen, z. B. Name, Adresse, E-Mail-Adresse oder Ihr Nutzerverhalten. Die Datenschutzerklärung gilt für alle von uns vorgenommene Daten-Verarbeitungsvorgänge sowohl im Rahmen unserer Kerntätigkeit als auch für die von uns vorgehaltenen Online-Medien." }
            </CosmoParagraph>
            <CosmoHeader
                level={CosmoHeaderLevel::H2}
                header="Wer bei uns für die Datenverarbeitung verantwortlich ist"
            />
            <CosmoParagraph>{ "Verantwortlich für die Datenverarbeitung ist:" }</CosmoParagraph>
            <address class={address_style}>
                { "Imanuel Ulbricht" }
                <br />
                { "Dingworthstr. 15" }
                <br />
                { "31137 Hildesheim" }
                <br />
                { "Telefon: +49 1525 5709066" }
                <br />
                { "E-Mail: " }
                <CosmoAnchor href="mailto:panda.helferlein@bambushain.app">
                    { "panda.helferlein@bambushain.app" }
                </CosmoAnchor>
            </address>
            <CosmoParagraph>
                { "Nach der DSGVO stehen Ihnen die nachfolgend aufgeführten Rechte zu, die Sie jederzeit bei dem in Ziffer 1. dieser Datenschutzerklärung genannten Verantwortlichen geltend machen können:" }
            </CosmoParagraph>
            <CosmoKeyValueList>
                <CosmoKeyValueListItem title="Recht auf Auskunft">
                    { "Sie haben das Recht, von uns Auskunft darüber zu verlangen, ob und welche Daten wir von Ihnen verarbeiten." }
                </CosmoKeyValueListItem>
                <CosmoKeyValueListItem title="Recht auf Berichtigung">
                    { "Sie haben das Recht, die Berichtigung unrichtiger oder Vervollständigung unvollständiger Daten zu verlangen." }
                </CosmoKeyValueListItem>
                <CosmoKeyValueListItem title="Recht auf Löschung">
                    { "Sie haben das Recht, die Löschung Ihrer Daten zu verlangen." }
                </CosmoKeyValueListItem>
                <CosmoKeyValueListItem title="Recht auf Einschränkung">
                    { "Sie haben in bestimmten Fällen das Recht zu verlangen, dass wir Ihre Daten nur noch eingeschränkt bearbeiten." }
                </CosmoKeyValueListItem>
                <CosmoKeyValueListItem title="Recht auf Datenübertragbarkeit">
                    { "Sie haben das Recht zu verlangen, dass wir Ihnen oder einem anderen Verantwortlichen Ihre Daten in einem strukturierten, gängigen und maschinenlesebaren Format übermitteln." }
                </CosmoKeyValueListItem>
                <CosmoKeyValueListItem title="Beschwerderecht">
                    { "Sie haben das Recht, sich bei einer Aufsichtsbehörde zu beschweren. Zuständig ist die Aufsichtsbehörde Ihres üblichen Aufenthaltsortes, Ihres Arbeitsplatzes oder unseres Firmensitzes." }
                </CosmoKeyValueListItem>
            </CosmoKeyValueList>
            <CosmoHeader level={CosmoHeaderLevel::H3} header="Widerrufsrecht" />
            <CosmoParagraph>
                { "Sie haben das Recht, die von Ihnen erteilte Einwilligung zur Datenverarbeitung jederzeit zu widerrufen." }
            </CosmoParagraph>
            <CosmoHeader level={CosmoHeaderLevel::H3} header="Widerspruchsrecht" />
            <CosmoParagraph>
                { "Sie haben das Recht, jederzeit gegen die Verarbeitung Ihrer Daten, die wir auf unser berechtigtes Interesse nach Art. 6 Abs. 1 lit. f DSGVO stützen, Widerspruch einzulegen. Sofern Sie von Ihrem Widerspruchsrecht Gebrauch machen, bitten wir Sie um die Darlegung der Gründe. Wir werden Ihre personenbezogenen Daten dann nicht mehr verarbeiten, es sei denn, wir können Ihnen gegenüber nachweisen, dass zwingende schutzwürdige Gründe an der Datenverarbeitung Ihre Interessen und Rechte überwiegen." }
            </CosmoParagraph>
            <CosmoParagraph>
                { "Unabhängig vom vorstehend Gesagten, haben Sie das jederzeitige Recht, der Verarbeitung Ihrer personenbezogenen Daten für Zwecke der Werbung und Datenanalyse zu widersprechen." }
            </CosmoParagraph>
            <CosmoParagraph>
                { "Ihren Widerspruch richten Sie bitte an die oben angegebene Kontaktadresse des Verantwortlichen." }
            </CosmoParagraph>
            <CosmoHeader level={CosmoHeaderLevel::H2} header="Wann löschen wir Ihre Daten?" />
            <CosmoParagraph>
                { "Wir löschen Ihre Daten dann, wenn wir diese nicht mehr brauchen oder Sie uns dies vorgeben. Das bedeutet, dass - sofern sich aus den einzelnen Datenschutzhinweisen dieser Datenschutzerklärung nichts anderes ergibt - wir Ihre Daten löschen," }
            </CosmoParagraph>
            <ul>
                <li>
                    { "wenn der Zweck der Datenverarbeitung weggefallen ist und damit die jeweilige in den einzelnen Datenschutzhinweisen genannte Rechtsgrundlage nicht mehr besteht, also bspw." }
                    <ul>
                        <li>
                            { "nach Beendigung der zwischen uns bestehenden vertraglichen oder mitgliedschaftlichen Beziehungen (Art. 6 Abs. 1 lit. a DSGVO) oder" }
                        </li>
                        <li>
                            { "nach Wegfall unseres berechtigten Interesses an der weiteren Verarbeitung oder Speicherung Ihrer Daten (Art. 6 Abs. 1 lit. f DSGVO)," }
                        </li>
                    </ul>
                </li>
                <li>
                    { "wenn Sie von Ihrem Widerrufsrecht Gebrauch machen und keine anderweitige gesetzliche Rechtsgrundlage für die Verarbeitung im Sinne von Art. 6 Abs. 1 lit. b-f DSGVO eingreift," }
                </li>
                <li>
                    { "wenn Sie vom Ihrem Widerspruchsrecht Gebrauch machen und der Löschung keine zwingenden schutzwürdigen Gründe entgegenstehen." }
                </li>
            </ul>
            <CosmoParagraph>
                { "Sofern wir (bestimmte Teile) Ihre(r) Daten jedoch noch für andere Zwecke vorhalten müssen, weil dies etwa steuerliche Aufbewahrungsfristen (in der Regel 6 Jahre für Geschäftskorrespondenz bzw. 10 Jahre für Buchungsbelege) oder die Geltendmachung, Ausübung oder Verteidigung von Rechtsansprüchen aus vertraglichen Beziehungen (bis zu vier Jahren) erforderlich machen oder die Daten zum Schutz der Rechte einer anderen natürlichen oder juristischen Person gebraucht werden, löschen wir (den Teil) Ihre(r) Daten erst nach Ablauf dieser Fristen. Bis zum Ablauf dieser Fristen beschränken wir die Verarbeitung dieser Daten jedoch auf diese Zwecke (Erfüllung der Aufbewahrungspflichten)." }
            </CosmoParagraph>
            <CosmoParagraph>
                { "Unsere Internetseite nutzt Cookies. Bei Cookies handelt es sich um kleine Textdateien, bestehend aus einer Reihe von Zahlen und Buchstaben, die auf dem von Ihnen genutzten Endgerät abgelegt und gespeichert werden. Cookies dienen vorrangig dazu, Informationen zwischen dem von Ihnen genutzten Endgerät und unserer Webseite auszutauschen. Hierzu gehören u. a. die Spracheinstellungen auf einer Webseite, der Login-Status oder die Stelle, an der ein Video geschaut wurde." }
            </CosmoParagraph>
            <CosmoParagraph>
                { "Beim Besuch unserer Webseiten wird ein permanenter Cookie eingesetzt, dieser Cookie speichert lediglich die Information für den Login und wird für den Kalender benötigt." }
            </CosmoParagraph>
            <CosmoHeader level={CosmoHeaderLevel::H2} header="Kontaktaufnahme" />
            <CosmoParagraph>
                { "Soweit Sie uns über E-Mail, Telefon, Post, unser Kontaktformular oder sonstwie ansprechen und uns hierbei personenbezogene Daten wie Ihren Namen, Ihre Telefonnummer oder Ihre E-Mail-Adresse zur Verfügung stellen oder weitere Angaben zur Ihrer Person oder Ihrem Anliegen machen, verarbeiten wir diese Daten zur Beantwortung Ihrer Anfrage im Rahmen des zwischen uns bestehenden vorvertraglichen oder vertraglichen Beziehungen." }
            </CosmoParagraph>
            <CosmoKeyValueList>
                <CosmoKeyValueListItem title="Betroffene Daten">
                    <ul>
                        <li>{ "Bestandsdaten (bspw. Namen, Adressen)" }</li>
                        <li>
                            { "Kontakdaten (bspw. E-Mail-Adresse, Telefonnummer, Postanschrift)" }
                        </li>
                        <li>{ "Inhaltsdaten (Texte, Fotos, Videos)" }</li>
                        <li>{ "Vertragsdaten (bspw. Vertragsgegenstand, Vertragsdauer)" }</li>
                    </ul>
                </CosmoKeyValueListItem>
                <CosmoKeyValueListItem title="Betroffene Personen">
                    { "Interessenten, Kunden, Geschäfts- und Vertragspartner" }
                </CosmoKeyValueListItem>
                <CosmoKeyValueListItem title="Verarbeitungszweck">
                    { "Kommunikation sowie Beantwortung von Kontaktanfragen, Büro und Organisationsverfahren" }
                </CosmoKeyValueListItem>
                <CosmoKeyValueListItem title="Rechtsgrundlage">
                    { "Vertragserfüllung und vorvertragliche Anfragen, Art. 6 Abs. 1 lit. b DSGVO, berechtigtes Interesse, Art. 6 Abs. 1 lit. f DSGVO" }
                </CosmoKeyValueListItem>
            </CosmoKeyValueList>
            <CosmoHeader level={CosmoHeaderLevel::H2} header="Sicherheitsmaßnahmen" />
            <CosmoParagraph>
                { "Wir treffen im Übrigen technische und organisatorische Sicherheitsmaßnahmen nach dem Stand der Technik, um die Vorschriften der Datenschutzgesetze einzuhalten und Ihre Daten gegen zufällige oder vorsätzliche Manipulationen, teilweisen oder vollständigen Verlust, Zerstörung oder gegen den unbefugten Zugriff Dritter zu schützen." }
            </CosmoParagraph>
            <CosmoHeader
                level={CosmoHeaderLevel::H2}
                header="Aktualität und Änderung dieser Datenschutzerklärung"
            />
            <CosmoParagraph>
                { "Diese Datenschutzerklärung ist aktuell gültig und hat den Stand Dezember 2023. Aufgrund geänderter gesetzlicher bzw. behördlicher Vorgaben kann es notwendig werden, diese Datenschutzerklärung anzupassen." }
            </CosmoParagraph>
            <CosmoParagraph>
                { "Diese Datenschutzerklärung wurde mit Hilfe des Datenschutz-Generators von SOS Recht erstellt. SOS Recht ist ein Angebot der Mueller.legal Rechtsanwälte Partnerschaft mit Sitz in Berlin." }
            </CosmoParagraph>
        </>
    )
}

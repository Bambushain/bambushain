use stylist::yew::use_style;
use yew::prelude::*;
use yew_cosmo::prelude::*;

#[function_component(ImagesPage)]
pub fn images() -> Html {
    let figure_style = use_style!(
        r#"
display: flex;
flex-flow: column;
background: var(--modal-backdrop);
backdrop-filter: var(--modal-container-backdrop-filter);
padding: 1rem;
width: 52rem;
gap: 1rem;
margin: 0;
"#
    );
    let img_style = use_style!(
        r#"
max-width: 50rem;
height: auto;
object-fit: scale-down;
"#
    );

    html!(
        <CosmoSideList>
            <CosmoSideListItem label="Hintergrundbild">
                <CosmoTitle title="Hintergrundbild" />
                <CosmoParagraph>
                    { "Bambushain hat ein Hintergrundbild von " }
                    <CosmoAnchor href="https://unsplash.com">{ "Unsplash" }</CosmoAnchor>
                    { ". Das Bild steht unter der Unsplash Lizenz, die wir unten verlinkt und kopiert haben. Das Bild haben wir auch verlinkt und noch einmal ohne Filter gezeigt." }
                </CosmoParagraph>
                <CosmoHeader level={CosmoHeaderLevel::H2} header="Bild" />
                <figure class={figure_style.clone()}>
                    <img src="/static/background.webp" class={img_style.clone()} />
                    <figcaption>
                        <CosmoAnchor
                            href="https://unsplash.com/de/fotos/ein-holzsteg-mit-pflanzen-und-baumen-l3lxbhkTApM"
                        >
                            { "Bildherkunft" }
                        </CosmoAnchor>
                    </figcaption>
                </figure>
                <CosmoHeader level={CosmoHeaderLevel::H2} header="Lizenz" />
                <CosmoParagraph>
                    { "Unsplash gewährt Ihnen eine unwiderrufliche, nicht-exklusive, weltweite Urheberrechtslizenz, um Fotos von Unsplash kostenlos herunterzuladen, zu kopieren, zu verändern, zu verbreiten, aufzuführen und zu nutzen, auch für kommerzielle Zwecke, ohne Genehmigung oder Nennung des Fotografen oder Unsplash. Diese Lizenz umfasst nicht das Recht, Fotos von Unsplash zu kompilieren, um einen ähnlichen oder konkurrierenden Service zu replizieren." }
                </CosmoParagraph>
                <CosmoParagraph>
                    { "Du findest die Unsplash Lizenz hier: " }
                    <CosmoAnchor href="https://unsplash.com/de/lizenz">
                        { "Unsplash Lizenz" }
                    </CosmoAnchor>
                </CosmoParagraph>
            </CosmoSideListItem>
            <CosmoSideListItem label="Login Hintergrundbild">
                <CosmoTitle title="Login Hintergrundbild" />
                <CosmoParagraph>
                    { "Bambushain hat ein Login Hintergrundbild von " }
                    <CosmoAnchor href="https://unsplash.com">{ "Unsplash" }</CosmoAnchor>
                    { ". Das Bild steht unter der Unsplash Lizenz, die wir unten verlinkt und kopiert haben. Das Bild haben wir auch verlinkt und noch einmal ohne Filter gezeigt." }
                </CosmoParagraph>
                <CosmoHeader level={CosmoHeaderLevel::H2} header="Bild" />
                <figure class={figure_style.clone()}>
                    <img src="/static/background-login.webp" class={img_style.clone()} />
                    <figcaption>
                        <CosmoAnchor
                            href="https://unsplash.com/de/fotos/haus-zwischen-wald-F-6rwMp1M-Q"
                        >
                            { "Bildherkunft" }
                        </CosmoAnchor>
                    </figcaption>
                </figure>
                <CosmoHeader level={CosmoHeaderLevel::H2} header="Lizenz" />
                <CosmoParagraph>
                    { "Unsplash gewährt Ihnen eine unwiderrufliche, nicht-exklusive, weltweite Urheberrechtslizenz, um Fotos von Unsplash kostenlos herunterzuladen, zu kopieren, zu verändern, zu verbreiten, aufzuführen und zu nutzen, auch für kommerzielle Zwecke, ohne Genehmigung oder Nennung des Fotografen oder Unsplash. Diese Lizenz umfasst nicht das Recht, Fotos von Unsplash zu kompilieren, um einen ähnlichen oder konkurrierenden Service zu replizieren." }
                </CosmoParagraph>
                <CosmoParagraph>
                    { "Du findest die Unsplash Lizenz hier: " }
                    <CosmoAnchor href="https://unsplash.com/de/lizenz">
                        { "Unsplash Lizenz" }
                    </CosmoAnchor>
                </CosmoParagraph>
            </CosmoSideListItem>
            <CosmoSideListItem label="Logo">
                <CosmoTitle title="Logo" />
                <CosmoParagraph>
                    { "Das aktuelle Bambushain Logo ist von " }
                    <CosmoAnchor href="https://www.svgrepo.com/">{ "SVG Repo" }</CosmoAnchor>
                    { ". Das Bild steht unter der CC0 Lizenz, die wir unten verlinkt und kopiert haben. Das Bild haben wir auch verlinkt und noch einmal ohne Filter gezeigt." }
                </CosmoParagraph>
                <CosmoHeader level={CosmoHeaderLevel::H2} header="Bild" />
                <figure class={figure_style}>
                    <img src="/static/favicon.svg" class={img_style} />
                    <figcaption>
                        <CosmoAnchor href="https://www.svgrepo.com/svg/220234/bamboo">
                            { "Bildherkunft" }
                        </CosmoAnchor>
                    </figcaption>
                </figure>
                <CosmoHeader level={CosmoHeaderLevel::H2} header="Lizenz" />
                <CosmoHeader level={CosmoHeaderLevel::H3} header="Public Domain" />
                <CosmoParagraph>
                    <em>
                        { "(As visual work) This license also might be referred as No copyright or CC0 1.0 Universal PD Dedication on our website." }
                    </em>
                </CosmoParagraph>
                <CosmoParagraph>
                    { "The person who associated a work with this deed has dedicated the work to the public domain by waiving all of his or her rights to the work worldwide under copyright law, including all related and neighboring rights, to the extent allowed by law." }
                </CosmoParagraph>
                <CosmoParagraph>
                    { "Or, the work consists of simple geometry and is not ineligable for copyright due to Threshold of originality (this threshold might vary depending on different country laws). For an example \"A stick figure, where the head is represented by a circle and other parts represented by straight lines\" is not copyrightable or falls into public domain." }
                </CosmoParagraph>
                <CosmoParagraph>{ "You are free:" }</CosmoParagraph>
                <ul>
                    <li>
                        <strong>{ "to share" }</strong>
                        { " – to copy, distribute and transmit the work" }
                    </li>
                    <li>
                        <strong>{ "to remix" }</strong>
                        { " – to adapt the work" }
                    </li>
                </ul>
                <CosmoParagraph>{ "Under the following terms:" }</CosmoParagraph>
                <ul>
                    <li>
                        <strong>{ "attribution" }</strong>
                        { " – there is no author or author waived their right, no attribution" }
                    </li>
                    <li>
                        <strong>{ "share alike" }</strong>
                        { " – If you remix, transform, or build upon the material, you can distribute your work under any license." }
                    </li>
                </ul>
                <CosmoParagraph>
                    { "Unless in the single pages of icons indexed on this website indicates differently, simple icons distributed on this website is subject to public domain or open source." }
                </CosmoParagraph>
                <CosmoParagraph>
                    { "To learn more about this license, " }
                    <CosmoAnchor href="https://creativecommons.org/publicdomain/zero/1.0/deed.en">
                        { "check out this page" }
                    </CosmoAnchor>
                    { "." }
                </CosmoParagraph>
                <CosmoParagraph>
                    { "Du findest die CC0 Lizenz hier: " }
                    <CosmoAnchor href="https://www.svgrepo.com/page/licensing/#CC0">
                        { "CC0 Lizenz" }
                    </CosmoAnchor>
                </CosmoParagraph>
            </CosmoSideListItem>
        </CosmoSideList>
    )
}

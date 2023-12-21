use yew::prelude::*;
use yew_cosmo::prelude::*;
use yew_hooks::prelude::{use_async, use_mount};

use crate::api;

#[function_component(SoftwareLicensesPage)]
pub fn licenses() -> Html {
    let licenses = use_async(async move { api::get_licenses().await });

    {
        let licenses = licenses.clone();
        use_mount(move || licenses.run())
    }

    if licenses.loading {
        html!()
    } else if let Some(licenses) = &licenses.data {
        html!(
            <CosmoSideList>
                {for licenses.into_iter().map(|license| {
                    CosmoSideListItem::from_label_and_children(license.name.clone().into(), html!(
                        <>
                            <CosmoTitle title={license.name.clone()} />
                            <CosmoKeyValueList>
                                if !license.authors.is_empty() {
                                    <CosmoKeyValueListItem title="Erstellt von">
                                        {license.authors.clone()}
                                    </CosmoKeyValueListItem>
                                }
                                if !license.description.is_empty() {
                                    <CosmoKeyValueListItem title="Beschreibung">
                                        {license.description.clone()}
                                    </CosmoKeyValueListItem>
                                }
                                if !license.license.is_empty() {
                                    <CosmoKeyValueListItem title="Lizenz">
                                        {license.license.clone()}
                                    </CosmoKeyValueListItem>
                                }
                                if !license.repository.is_empty() {
                                    <CosmoKeyValueListItem title="Repository">
                                        <CosmoAnchor href={license.repository.clone()}>{license.repository.clone()}</CosmoAnchor>
                                    </CosmoKeyValueListItem>
                                }
                            </CosmoKeyValueList>
                        </>
                    ))
                })}
            </CosmoSideList>
        )
    } else {
        html! {}
    }
}

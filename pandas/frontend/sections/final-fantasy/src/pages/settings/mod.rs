use yew::prelude::*;
use yew_cosmo::prelude::*;

use crate::pages::settings::custom_fields::CustomFieldsPage;
use crate::pages::settings::free_companies::FreeCompaniesPage;

mod custom_fields;
mod free_companies;

#[function_component(SettingsPage)]
pub fn settings_page() -> Html {
    html!(
        <CosmoSideList>
            <CosmoSideListItem label="Eigene Felder">
                <CustomFieldsPage />
            </CosmoSideListItem>
            <CosmoSideListItem label="Freie Gesellschaften">
                <FreeCompaniesPage />
            </CosmoSideListItem>
        </CosmoSideList>
    )
}

use yew::prelude::*;

use bamboo_entities::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ModifyCharacterModalProps {
    pub on_close: Callback<()>,
    pub title: AttrValue,
    pub save_label: AttrValue,
    pub error_message: AttrValue,
    pub has_error: bool,
    pub has_unknown_error: bool,
    #[prop_or_default]
    pub character: Character,
    pub on_save: Callback<Character>,
    pub on_error_close: Callback<()>,
    pub custom_fields: Vec<CustomCharacterField>,
    pub free_companies: Vec<FreeCompany>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct CharacterDetailsProps {
    pub character: Character,
    pub on_delete: Callback<()>,
    pub custom_fields: Vec<CustomCharacterField>,
    pub free_companies: Vec<FreeCompany>,
}

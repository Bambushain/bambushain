use yew::prelude::*;

use bamboo_entities::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct HousingDetailsProps {
    pub character: Character,
}

#[derive(Properties, PartialEq, Clone)]
pub struct ModifyHousingModalProps {
    pub on_close: Callback<()>,
    pub on_error_close: Callback<()>,
    pub title: AttrValue,
    pub save_label: AttrValue,
    pub error_message: AttrValue,
    pub has_error: bool,
    pub has_unknown_error: bool,
    #[prop_or_default]
    pub housing: CharacterHousing,
    pub character_id: i32,
    pub on_save: Callback<CharacterHousing>,
    pub is_edit: bool,
}

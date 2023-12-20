use yew::prelude::*;

use bamboo_entities::prelude::{Character, Crafter, CrafterJob};

#[derive(Properties, PartialEq, Clone)]
pub struct CrafterDetailsProps {
    pub character: Character,
}

#[derive(Properties, PartialEq, Clone)]
pub struct ModifyCrafterModalProps {
    pub on_close: Callback<()>,
    pub on_error_close: Callback<()>,
    pub title: AttrValue,
    pub save_label: AttrValue,
    pub error_message: AttrValue,
    pub has_error: bool,
    pub has_unknown_error: bool,
    #[prop_or_default]
    pub crafter: Crafter,
    pub character_id: i32,
    pub on_save: Callback<Crafter>,
    pub is_edit: bool,
    pub jobs: Vec<CrafterJob>,
}

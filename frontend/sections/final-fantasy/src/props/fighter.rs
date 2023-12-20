use yew::prelude::*;

use bamboo_entities::prelude::{Character, Fighter, FighterJob};

#[derive(Properties, PartialEq, Clone)]
pub struct FighterDetailsProps {
    pub character: Character,
}

#[derive(Properties, PartialEq, Clone)]
pub struct ModifyFighterModalProps {
    pub on_close: Callback<()>,
    pub on_error_close: Callback<()>,
    pub title: AttrValue,
    pub save_label: AttrValue,
    pub error_message: AttrValue,
    pub has_error: bool,
    pub has_unknown_error: bool,
    #[prop_or_default]
    pub fighter: Fighter,
    pub character_id: i32,
    pub on_save: Callback<Fighter>,
    pub is_edit: bool,
    pub jobs: Vec<FighterJob>,
}

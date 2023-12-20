use yew::prelude::*;

use bamboo_entities::prelude::*;

#[derive(PartialEq, Clone, Properties)]
pub struct FieldsTabItemProps {
    pub field: CustomCharacterField,
    pub on_change: Callback<()>,
    pub on_move: Callback<usize>,
    pub is_last: bool,
    pub is_first: bool,
    pub position: i32,
}

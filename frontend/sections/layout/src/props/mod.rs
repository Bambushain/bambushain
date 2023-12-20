use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ChangePasswordDialogProps {
    pub on_close: Callback<()>,
    pub mods: Vec<AttrValue>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct UpdateMyProfileDialogProps {
    pub on_close: Callback<()>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct EnableTotpDialogProps {
    pub on_close: Callback<()>,
}

use bamboo_entities::prelude::WebUser;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct UserDetailsProps {
    pub user: WebUser,
    pub on_delete: Callback<()>,
    pub on_update: Callback<()>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct CreateUserModalProps {
    pub on_saved: Callback<WebUser>,
    pub on_close: Callback<()>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct UpdateProfileDialogProps {
    pub on_update: Callback<()>,
    pub on_close: Callback<()>,
    pub display_name: AttrValue,
    pub email: AttrValue,
    pub discord_name: AttrValue,
    pub id: i32,
}

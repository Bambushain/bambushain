use crate::BambooDialogs;
use yew::prelude::*;

#[hook]
pub fn use_dialogs() -> BambooDialogs {
    use_context::<BambooDialogs>().unwrap()
}

use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginCallbackQuery {
    pub state: String,
    pub code: String,
}

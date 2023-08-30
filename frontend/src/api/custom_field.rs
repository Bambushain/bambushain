use pandaparty_entities::prelude::CustomCharacterField;
use crate::api::{get, PandapartyApiResult};

pub async fn get_custom_fields() -> PandapartyApiResult<Vec<CustomCharacterField>> {
    log::debug!("Get custom fields");
    get("/api/final-fantasy/character/custom-field").await
}
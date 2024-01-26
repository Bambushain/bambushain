use actix_web::{body, dev, web, Error, HttpMessage};
use actix_web_lab::middleware::Next;

use bamboo_common::backend::dbal;
use bamboo_common::backend::response::*;
use bamboo_common::backend::services::DbConnection;
use bamboo_common::core::entities;

use crate::header;
use crate::middleware::helpers;
use crate::path;

pub(crate) async fn extract_character(
    path: Option<path::CharacterPath>,
    db: DbConnection,
    authorization: Option<web::Header<header::AuthorizationHeader>>,
    req: dev::ServiceRequest,
    next: Next<impl body::MessageBody>,
) -> Result<dev::ServiceResponse<impl body::MessageBody>, Error> {
    let (_, user) = helpers::get_user_and_token_by_header(&db, authorization).await?;

    let path = check_invalid_path!(path, "character")?;
    let character = dbal::get_character(path.character_id, user.id, &db).await?;

    req.extensions_mut().insert(character);

    next.call(req).await
}

macro_rules! character {
    () => {
        actix_web_lab::middleware::from_fn(crate::middleware::extract_character::extract_character)
    };
}

pub(crate) type CharacterData = web::ReqData<entities::Character>;

pub(crate) use character;

use actix_web::get;

use bamboo_entities::prelude::Grove;
use bamboo_error::BambooApiResult;

use crate::middleware::authenticate_user::authenticate;
use crate::middleware::identify_grove::{grove, CurrentGrove};
use crate::response::macros::ok;

#[get("/api/grove", wrap = "authenticate!()", wrap = "grove!()")]
pub async fn get_grove(current_grove: CurrentGrove) -> BambooApiResult<Grove> {
    Ok(ok!(current_grove.grove.clone()))
}

use actix_web::get;

use bamboo_common::backend::dbal;
use bamboo_common::backend::response::list;
use bamboo_common::backend::services::DbConnection;
use bamboo_common::core::error::BambooApiResponseResult;

use crate::middleware::authenticate_user::authenticate;

#[get("/api/grove", wrap = "authenticate!()")]
pub async fn get_groves(db: DbConnection) -> BambooApiResponseResult {
    dbal::get_groves(&db).await.map(|data| list!(data))
}

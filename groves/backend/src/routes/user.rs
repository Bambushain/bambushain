use actix_web::{get, web};

use bamboo_common::backend::dbal;
use bamboo_common::backend::response::{check_invalid_path, list};
use bamboo_common::backend::services::DbConnection;
use bamboo_common::core::entities::GroveUser;
use bamboo_common::core::error::BambooApiResponseResult;

use crate::middleware::authenticate_user::authenticate;
use crate::path::GrovePath;

#[get("/api/grove/{grove_id}/user", wrap = "authenticate!()")]
pub async fn get_users(
    path: Option<web::Path<GrovePath>>,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "user")?;
    dbal::get_users_filtered_for_management(path.grove_id, &db)
        .await
        .map(|data| {
            list!(data
                .into_iter()
                .map(GroveUser::from)
                .collect::<Vec<GroveUser>>())
        })
}

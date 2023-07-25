use log::LevelFilter;
use sea_orm::{ConnectOptions, Database};
use sea_orm::prelude::*;

macro_rules! open_db_connection {
    () => {
        match crate::get_database_connection().await {
            Ok(db) => db,
            Err(err) => {
                log::error!("Failed to open database connection: {err}");
                return Err(sheef_entities::sheef_db_error!("database", "Failed to open database connection"))
            }
        }
    };
}

macro_rules! get_user_by_username {
    ($username:expr) => {
        {
            match crate::user::get_user($username.clone()).await {
                Ok(user) => user,
                Err(err) => {
                    log::error!("Failed to load user {}: {err}", $username);
                    return Err(sheef_entities::sheef_not_found_error!("user", "User not found"));
                }
            }
        }
    };
}

pub mod authentication;
pub mod calendar;
pub mod crafter;
pub mod fighter;
pub mod kill;
pub mod user;
pub mod mount;
pub mod savage_mount;

pub async fn get_database_connection() -> Result<DatabaseConnection, DbErr> {
    let mut opts = ConnectOptions::new(std::env::var("DATABASE_URL").expect("Needs DATABASE_URL"));
    opts.sqlx_logging(true)
        .sqlx_logging_level(LevelFilter::Debug);

    Database::connect(opts).await
}

pub mod prelude {
    pub use crate::authentication::*;
    pub use crate::calendar::*;
    pub use crate::crafter::*;
    pub use crate::fighter::*;
    pub use crate::kill::*;
    pub use crate::mount::*;
    pub use crate::savage_mount::*;
    pub use crate::user::*;
}

#[macro_export]
macro_rules! open_db_connection {
    () => {
        {
            let mut opts = sea_orm::ConnectOptions::new(std::env::var("DATABASE_URL").expect("Needs DATABASE_URL"));
            opts.sqlx_logging(true)
                .sqlx_logging_level(log::LevelFilter::Info);

            match sea_orm::Database::connect(opts).await {
                Ok(db) => db,
                Err(err) => {
                    log::error!("Failed to open database connection: {err}");
                    return Err(pandaparty_entities::pandaparty_db_error!("database", "Failed to open database connection"))
                }
            }
        }
    };
}

#[macro_export]
macro_rules! open_db_connection_with_error {
    () => {
        {
            let mut opts = sea_orm::ConnectOptions::new(std::env::var("DATABASE_URL").expect("Needs DATABASE_URL"));
            opts.sqlx_logging(true)
                .sqlx_logging_level(log::LevelFilter::Debug);

            sea_orm::Database::connect(opts).await
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

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

pub async fn get_database() -> Result<DatabaseConnection, DbErr> {
    let mut opts = ConnectOptions::new(std::env::var("DATABASE_URL").expect("Needs DATABASE_URL"));
    opts.sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Debug);

    Database::connect(opts).await
}

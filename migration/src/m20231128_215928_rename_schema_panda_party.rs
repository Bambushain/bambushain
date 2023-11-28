use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let bamboo_stmt = Statement::from_string(
            manager.get_database_backend(),
            "ALTER SCHEMA panda_party RENAME TO bamboo",
        );

        db.execute(bamboo_stmt).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let bamboo_stmt = Statement::from_string(
            manager.get_database_backend(),
            "ALTER SCHEMA bamboo RENAME TO panda_party",
        );

        db.execute(bamboo_stmt).await?;

        Ok(())
    }
}

use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::Statement;

use crate::m20220101_000001_create_schemas::Schemas;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let grove_stmt =
            Statement::from_string(manager.get_database_backend(), "CREATE SCHEMA grove");
        db.execute(grove_stmt).await?;

        manager
            .create_table(
                Table::create()
                    .table((Schemas::Grove, Grove::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Grove::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Grove::Name).string().not_null().unique_key())
                    .col(
                        ColumnDef::new(Grove::IsSuspended)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Grove::IsEnabled)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let grove_stmt =
            Statement::from_string(manager.get_database_backend(), "DROP SCHEMA grove");
        db.execute(grove_stmt).await?;

        manager
            .drop_table(
                Table::drop()
                    .table((Schemas::Grove, Grove::Table))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum Grove {
    Table,
    Id,
    Name,
    IsSuspended,
    IsEnabled,
}

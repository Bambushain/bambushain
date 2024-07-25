use crate::m20220101_000001_create_schemas::Schemas;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table((Schemas::Grove, GroveUser::Table))
                    .add_column(ColumnDef::new(GroveUser::IsBanned).boolean().default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table((Schemas::Grove, GroveUser::Table))
                    .drop_column(GroveUser::IsBanned)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum GroveUser {
    Table,
    IsBanned,
}

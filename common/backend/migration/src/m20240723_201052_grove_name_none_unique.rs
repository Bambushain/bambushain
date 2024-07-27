use crate::m20220101_000001_create_schemas::Schemas;
use crate::m20231229_235511_create_table_grove::Grove;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table((Schemas::Grove, Grove::Table))
                    .modify_column(ColumnDef::new(Grove::Name).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table((Schemas::Grove, Grove::Table))
                    .modify_column(ColumnDef::new(Grove::Name).string().not_null().unique_key())
                    .to_owned(),
            )
            .await
    }
}

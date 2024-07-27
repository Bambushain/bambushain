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
                    .table((Schemas::Grove, Grove::Table))
                    .add_column(ColumnDef::new(Grove::InviteSecret).string().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table((Schemas::Grove, Grove::Table))
                    .drop_column(Grove::InviteSecret)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Grove {
    Table,
    InviteSecret,
}

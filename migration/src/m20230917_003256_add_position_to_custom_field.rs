use sea_orm_migration::prelude::*;
use crate::m20220101_000001_create_schemas::Schemas;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table((Schemas::FinalFantasy, CustomCharacterField::Table))
                    .add_column(
                        ColumnDef::new(CustomCharacterField::Position)
                            .integer()
                            .null()
                            .default(0)
                    )
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table((Schemas::FinalFantasy, CustomCharacterField::Table))
                    .drop_column(CustomCharacterField::Position)
                    .to_owned()
            )
            .await
    }
}

#[derive(DeriveIden)]
enum CustomCharacterField {
    Table,
    Position,
}

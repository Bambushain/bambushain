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
                    .table((Schemas::Authentication, User::Table))
                    .add_column(
                        ColumnDef::new(User::TotpSecret)
                            .binary()
                            .null()
                    )
                    .add_column(
                        ColumnDef::new(User::TotpValidated)
                            .boolean()
                            .null()
                    )
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .drop_column(User::TotpSecret)
                    .drop_column(User::TotpValidated)
                    .to_owned()
            )
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum User {
    Table,
    TotpSecret,
    TotpValidated,
}

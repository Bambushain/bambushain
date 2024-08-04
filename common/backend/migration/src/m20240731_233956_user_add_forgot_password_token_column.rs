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
                    .table((Schemas::Authentication, User::Table))
                    .add_column(ColumnDef::new(User::ForgotPasswordCode).string().null())
                    .add_column(ColumnDef::new(User::ForgotPasswordValidUntil).date().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table((Schemas::Authentication, User::Table))
                    .drop_column(User::ForgotPasswordCode)
                    .drop_column(User::ForgotPasswordValidUntil)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    ForgotPasswordCode,
    ForgotPasswordValidUntil,
}

use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_schemas::Schemas;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Schemas::Authentication, User::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(User::Email)
                            .string()
                            .not_null()
                            .unique_key()
                    )
                    .col(
                        ColumnDef::new(User::Password)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(User::DisplayName)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef
                        ::new(User::DiscordName)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef
                        ::new(User::TwoFactorCode)
                            .string()
                            .string_len(6)
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(User::IsMod)
                            .boolean()
                            .not_null()
                            .default(false)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Schemas::Authentication, User::Table))
                    .to_owned()
            )
            .await
    }
}

#[derive(Iden)]
pub enum User {
    Table,
    Id,
    DisplayName,
    Password,
    IsMod,
    DiscordName,
    Email,
    TwoFactorCode,
}

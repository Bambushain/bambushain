use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_schemas::Schemas;
use crate::m20230724_121011_create_table_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Schemas::FinalFantasy, CustomCharacterField::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CustomCharacterField::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CustomCharacterField::Label)
                        .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CustomCharacterField::UserId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from((Schemas::FinalFantasy, CustomCharacterField::Table), CustomCharacterField::UserId)
                            .to((Schemas::Authentication, User::Table), User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Schemas::FinalFantasy, CustomCharacterField::Table))
                    .to_owned(),
            )
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum CustomCharacterField {
    Table,
    Id,
    UserId,
    Label,
}

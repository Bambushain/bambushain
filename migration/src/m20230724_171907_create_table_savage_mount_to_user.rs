use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_table_user::User;
use crate::m20230724_171834_create_table_savage_mount::SavageMount;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SavageMountToUser::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(SavageMountToUser::UserId).integer().not_null())
                    .col(ColumnDef::new(SavageMountToUser::SavageMountId).integer().not_null())
                    .foreign_key(ForeignKey::create()
                        .from(SavageMountToUser::Table, SavageMountToUser::UserId)
                        .to(User::Table, User::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(ForeignKey::create()
                        .from(SavageMountToUser::Table, SavageMountToUser::SavageMountId)
                        .to(SavageMount::Table, SavageMount::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                    )
                    .primary_key(Index::create().col(SavageMountToUser::UserId).col(SavageMountToUser::SavageMountId))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SavageMountToUser::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum SavageMountToUser {
    Table,
    UserId,
    SavageMountId,
}
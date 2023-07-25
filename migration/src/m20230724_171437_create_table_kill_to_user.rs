use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_table_user::User;
use crate::m20230724_171039_create_table_kill::Kill;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(KillToUser::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(KillToUser::UserId).integer().not_null())
                    .col(ColumnDef::new(KillToUser::KillId).integer().not_null())
                    .foreign_key(ForeignKey::create()
                        .from(KillToUser::Table, KillToUser::UserId)
                        .to(User::Table, User::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(ForeignKey::create()
                        .from(KillToUser::Table, KillToUser::KillId)
                        .to(Kill::Table, Kill::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                    )
                    .primary_key(Index::create().col(KillToUser::UserId).col(KillToUser::KillId))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(KillToUser::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum KillToUser {
    Table,
    UserId,
    KillId,
}

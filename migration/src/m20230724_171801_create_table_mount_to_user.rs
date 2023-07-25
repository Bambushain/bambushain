use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_table_user::User;
use crate::m20230724_171715_create_table_mount::Mount;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MountToUser::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(MountToUser::UserId).integer().not_null())
                    .col(ColumnDef::new(MountToUser::MountId).integer().not_null())
                    .foreign_key(ForeignKey::create()
                        .from(MountToUser::Table, MountToUser::UserId)
                        .to(User::Table, User::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(ForeignKey::create()
                        .from(MountToUser::Table, MountToUser::MountId)
                        .to(Mount::Table, Mount::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                    )
                    .primary_key(Index::create().col(MountToUser::UserId).col(MountToUser::MountId))

                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MountToUser::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum MountToUser {
    Table,
    UserId,
    MountId,
}

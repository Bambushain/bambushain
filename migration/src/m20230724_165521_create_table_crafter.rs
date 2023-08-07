use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_table_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Crafter::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Crafter::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Crafter::Job).string().not_null())
                    .col(ColumnDef::new(Crafter::Level).string())
                    .col(ColumnDef::new(Crafter::UserId).integer().not_null())
                    .foreign_key(ForeignKey::create()
                        .from(Crafter::Table, Crafter::UserId)
                        .to(User::Table, User::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                    )
                    .index(Index::create()
                        .col(Crafter::Job)
                        .col(Crafter::UserId)
                        .unique()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Crafter::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Crafter {
    Table,
    Id,
    Job,
    Level,
    UserId,
}

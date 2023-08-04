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
                    .table(Event::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Event::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Event::UserId).integer().not_null())
                    .col(ColumnDef::new(Event::Time).string().not_null().default(false))
                    .col(ColumnDef::new(Event::Date).date().not_null())
                    .col(ColumnDef::new(Event::Available).boolean().not_null().default(false))
                    .index(Index::create()
                        .table(Event::Table)
                        .col(Event::UserId)
                        .col(Event::Date)
                        .unique())
                    .foreign_key(ForeignKey::create()
                        .from(Event::Table, Event::UserId)
                        .to(User::Table, User::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Event::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Event {
    Table,
    Id,
    UserId,
    Time,
    Date,
    Available,
}

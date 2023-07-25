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
                    .table(Fighter::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Fighter::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Fighter::Job).string().not_null())
                    .col(ColumnDef::new(Fighter::Level).string())
                    .col(ColumnDef::new(Fighter::GearScore).string())
                    .col(ColumnDef::new(Fighter::UserId).integer().not_null())
                    .foreign_key(ForeignKey::create()
                        .from(Fighter::Table, Fighter::UserId)
                        .to(User::Table, User::Id))
                    .index(Index::create()
                        .col(Fighter::Job)
                        .col(Fighter::UserId)
                        .unique()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Fighter::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Fighter {
    Table,
    Id,
    Job,
    Level,
    GearScore,
    UserId,
}
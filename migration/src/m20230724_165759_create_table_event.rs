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
                    .table((Schemas::PandaParty, Event::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Event::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Event::Title).string().not_null())
                    .col(
                        ColumnDef::new(Event::Description)
                            .text()
                            .not_null()
                            .default(""),
                    )
                    .col(ColumnDef::new(Event::StartDate).date().not_null())
                    .col(ColumnDef::new(Event::EndDate).date().not_null())
                    .col(ColumnDef::new(Event::Color).string().not_null())
                    .check(Expr::col(Event::EndDate).gte(Expr::col(Event::StartDate)))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Schemas::PandaParty, Event::Table))
                    .to_owned(),
            )
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Event {
    Table,
    Id,
    Title,
    StartDate,
    EndDate,
    Description,
    Color,
}

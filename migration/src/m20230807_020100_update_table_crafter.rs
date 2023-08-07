use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::EnumIter;
use sea_orm_migration::sea_query::extension::postgres::Type;

use crate::m20230724_165521_create_table_crafter::Crafter;
use crate::sea_orm::Iterable;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Crafter::Table)
                    .drop_column(Crafter::Job)
                    .to_owned(),
            )
            .await?;
        manager
            .create_type(
                Type::create()
                    .as_enum(Alias::new("crafter_job"))
                    .values(CrafterJob::iter().collect::<Vec<CrafterJob>>())
                    .to_owned()
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Crafter::Table)
                    .add_column(
                        ColumnDef::new(Crafter::Job)
                            .custom(Alias::new("crafter_job"))
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(Crafter::Table)
                    .col(Crafter::Job)
                    .col(Crafter::UserId)
                    .unique()
                    .to_owned()
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Crafter::Table)
                    .drop_column(Crafter::Job)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_type(
                Type::drop()
                    .name(Alias::new("crafter_job"))
                    .to_owned()
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(Crafter::Table)
                    .col(Crafter::Job)
                    .col(Crafter::UserId)
                    .unique()
                    .to_owned()
            )
            .await?;

        Ok(())
    }
}

#[derive(Iden, EnumIter)]
enum CrafterJob {
    Carpenter,
    Blacksmith,
    Armorer,
    Goldsmith,
    Leatherworker,
    Weaver,
    Alchemist,
    Culinarian,
    Miner,
    Botanist,
    Fisher,
}
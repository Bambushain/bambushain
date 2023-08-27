use sea_orm_migration::prelude::*;
use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::sea_orm::{EnumIter, Iterable};

use crate::m20220101_000001_create_schemas::Schemas;
use crate::m20230724_121111_create_table_character::Character;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(
                        (Schemas::FinalFantasy, Alias::new("crafter_job"))
                    )
                    .values(
                        CrafterJob::iter()
                            .collect::<Vec<CrafterJob>>()
                    )
                    .to_owned()
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table((Schemas::FinalFantasy, Crafter::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Crafter::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Crafter::Job)
                            .custom(Alias::new("final_fantasy.crafter_job"))
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Crafter::Level)
                            .string()
                    )
                    .col(
                        ColumnDef::new(Crafter::CharacterId)
                            .integer()
                            .not_null()
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from((Schemas::FinalFantasy, Crafter::Table), Crafter::CharacterId)
                            .to((Schemas::FinalFantasy, Character::Table), Character::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .index(
                        Index::create()
                            .col(Crafter::Job)
                            .col(Crafter::CharacterId)
                            .unique()
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Schemas::FinalFantasy, Crafter::Table))
                    .to_owned()
            )
            .await?;
        manager
            .drop_type(
                Type::drop()
                    .name((Schemas::FinalFantasy, Alias::new("crafter_job")))
                    .to_owned()
            )
            .await?;

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Crafter {
    Table,
    Id,
    Job,
    Level,
    CharacterId,
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

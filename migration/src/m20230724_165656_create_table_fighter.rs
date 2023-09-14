use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::prelude::*;
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
                    .as_enum((Schemas::FinalFantasy, Alias::new("fighter_job")))
                    .values(FighterJob::iter().collect::<Vec<FighterJob>>())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table((Schemas::FinalFantasy, Fighter::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Fighter::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Fighter::Job)
                            .custom(Alias::new("final_fantasy.fighter_job"))
                            .not_null(),
                    )
                    .col(ColumnDef::new(Fighter::Level).string())
                    .col(ColumnDef::new(Fighter::GearScore).string())
                    .col(ColumnDef::new(Fighter::CharacterId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                (Schemas::FinalFantasy, Fighter::Table),
                                Fighter::CharacterId,
                            )
                            .to((Schemas::FinalFantasy, Character::Table), Character::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .col(Fighter::Job)
                            .col(Fighter::CharacterId)
                            .unique(),
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
                    .table((Schemas::FinalFantasy, Fighter::Table))
                    .to_owned(),
            )
            .await?;
        manager
            .drop_type(
                Type::drop()
                    .name((Schemas::FinalFantasy, Alias::new("fighter_job")))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Fighter {
    Table,
    Id,
    Job,
    Level,
    GearScore,
    CharacterId,
}

#[derive(Iden, EnumIter)]
enum FighterJob {
    Paladin,
    Warrior,
    DarkKnight,
    Gunbreaker,
    WhiteMage,
    Scholar,
    Astrologian,
    Sage,
    Monk,
    Dragoon,
    Ninja,
    Samurai,
    Reaper,
    Bard,
    Machinist,
    Dancer,
    BlackMage,
    Summoner,
    RedMage,
    BlueMage,
}

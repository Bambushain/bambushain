use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::EnumIter;
use sea_orm_migration::sea_query::extension::postgres::Type;

use crate::m20230724_165656_create_table_fighter::Fighter;
use crate::sea_orm::Iterable;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Fighter::Table)
                    .drop_column(Fighter::Job)
                    .to_owned(),
            )
            .await?;
        manager
            .create_type(
                Type::create()
                    .as_enum(Alias::new("fighter_job"))
                    .values(FighterJob::iter().collect::<Vec<FighterJob>>())
                    .to_owned()
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Fighter::Table)
                    .add_column(
                        ColumnDef::new(Fighter::Job)
                            .custom(Alias::new("fighter_job"))
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(Fighter::Table)
                    .col(Fighter::Job)
                    .col(Fighter::UserId)
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
                    .table(Fighter::Table)
                    .drop_column(Fighter::Job)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_type(
                Type::drop()
                    .name(Alias::new("fighter_job"))
                    .to_owned()
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(Fighter::Table)
                    .col(Fighter::Job)
                    .col(Fighter::UserId)
                    .unique()
                    .to_owned()
            )
            .await?;

        Ok(())
    }
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
use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{EnumIter, Iterable};

use crate::m20220101_000001_create_schemas::Schemas;
use crate::m20230724_121011_create_table_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum((Schemas::FinalFantasy, Alias::new("character_race")))
                    .values(Race::iter().collect::<Vec<Race>>())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table((Schemas::FinalFantasy, Character::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Character::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Character::Name).string().not_null())
                    .col(
                        ColumnDef::new(Character::Race)
                            .custom(Alias::new("final_fantasy.character_race"))
                            .not_null(),
                    )
                    .col(ColumnDef::new(Character::World).string().not_null())
                    .col(ColumnDef::new(Character::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from((Schemas::FinalFantasy, Character::Table), Character::UserId)
                            .to((Schemas::Authentication, User::Table), User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
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
                    .table((Schemas::FinalFantasy, Character::Table))
                    .to_owned(),
            )
            .await?;
        manager
            .drop_type(
                Type::drop()
                    .name((Schemas::FinalFantasy, Alias::new("character_race")))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Character {
    Table,
    Id,
    Name,
    Race,
    World,
    UserId,
}

#[derive(Iden, EnumIter)]
enum Race {
    Hyur,
    Elezen,
    Lalafell,
    Miqote,
    Roegadyn,
    AuRa,
    Hrothgar,
    Viera,
}

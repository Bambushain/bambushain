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
                    .as_enum((Schemas::FinalFantasy, Alias::new("district")))
                    .values(HousingDistrict::iter().collect::<Vec<HousingDistrict>>())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table((Schemas::FinalFantasy, CharacterHousing::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CharacterHousing::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CharacterHousing::District)
                            .custom(Alias::new("final_fantasy.district"))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CharacterHousing::Ward)
                            .tiny_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CharacterHousing::Plot)
                            .tiny_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CharacterHousing::CharacterId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                (Schemas::FinalFantasy, CharacterHousing::Table),
                                CharacterHousing::CharacterId,
                            )
                            .to((Schemas::FinalFantasy, Character::Table), Character::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .col(CharacterHousing::District)
                            .col(CharacterHousing::Ward)
                            .col(CharacterHousing::Plot)
                            .col(CharacterHousing::CharacterId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Schemas::FinalFantasy, CharacterHousing::Table))
                    .to_owned(),
            )
            .await?;
        manager
            .drop_type(
                Type::drop()
                    .name((Schemas::FinalFantasy, Alias::new("district")))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum CharacterHousing {
    Table,
    Id,
    District,
    Ward,
    Plot,
    CharacterId,
}

#[derive(Iden, EnumIter)]
enum HousingDistrict {
    TheLavenderBeds,
    Mist,
    TheGoblet,
    Shirogane,
    Empyreum,
}

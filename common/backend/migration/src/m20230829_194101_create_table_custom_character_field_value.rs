use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_schemas::Schemas;
use crate::m20230724_121111_create_table_character::Character;
use crate::m20230829_194031_create_table_custom_character_field::CustomCharacterField;
use crate::m20230829_194055_create_table_custom_character_field_option::CustomCharacterFieldOption;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Schemas::FinalFantasy, CustomCharacterFieldValue::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CustomCharacterFieldValue::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CustomCharacterFieldValue::CharacterId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CustomCharacterFieldValue::CustomCharacterFieldId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CustomCharacterFieldValue::CustomCharacterFieldOptionId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                (Schemas::FinalFantasy, CustomCharacterFieldValue::Table),
                                CustomCharacterFieldValue::CustomCharacterFieldOptionId,
                            )
                            .to(
                                (Schemas::FinalFantasy, CustomCharacterFieldOption::Table),
                                CustomCharacterFieldOption::Id,
                            )
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                (Schemas::FinalFantasy, CustomCharacterFieldValue::Table),
                                CustomCharacterFieldValue::CustomCharacterFieldId,
                            )
                            .to(
                                (Schemas::FinalFantasy, CustomCharacterField::Table),
                                CustomCharacterField::Id,
                            )
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                (Schemas::FinalFantasy, CustomCharacterFieldValue::Table),
                                CustomCharacterFieldValue::CharacterId,
                            )
                            .to((Schemas::FinalFantasy, Character::Table), Character::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Schemas::FinalFantasy, CustomCharacterFieldValue::Table))
                    .to_owned(),
            )
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum CustomCharacterFieldValue {
    Table,
    Id,
    CustomCharacterFieldOptionId,
    CustomCharacterFieldId,
    CharacterId,
}

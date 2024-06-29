use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_schemas::Schemas;
use crate::m20230724_121111_create_table_character::Character;
use crate::m20230829_194101_create_table_custom_character_field_value::CustomCharacterFieldValue;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table((Schemas::FinalFantasy, CustomCharacterFieldValue::Table))
                    .name("custom_character_field_value_custom_character_field_id_fkey")
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(
                        (Schemas::FinalFantasy, CustomCharacterFieldValue::Table),
                        CustomCharacterFieldValue::CharacterId,
                    )
                    .to((Schemas::FinalFantasy, Character::Table), Character::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

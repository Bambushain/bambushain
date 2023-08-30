use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_schemas::Schemas;
use crate::m20230829_194031_create_table_custom_character_field::CustomCharacterField;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Schemas::FinalFantasy, CustomCharacterFieldOption::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CustomCharacterFieldOption::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CustomCharacterFieldOption::Label)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CustomCharacterFieldOption::CustomCharacterFieldId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from((Schemas::FinalFantasy, CustomCharacterFieldOption::Table), CustomCharacterFieldOption::CustomCharacterFieldId)
                            .to((Schemas::FinalFantasy, CustomCharacterField::Table), CustomCharacterField::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .table((Schemas::FinalFantasy, CustomCharacterFieldOption::Table))
                            .col(CustomCharacterFieldOption::CustomCharacterFieldId)
                            .col(CustomCharacterFieldOption::Label)
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
                    .table((Schemas::FinalFantasy, CustomCharacterFieldOption::Table))
                    .to_owned(),
            )
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum CustomCharacterFieldOption {
    Table,
    Id,
    CustomCharacterFieldId,
    Label,
}

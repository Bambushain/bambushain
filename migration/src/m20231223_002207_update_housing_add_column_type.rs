use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{EnumIter, Iterable};

use crate::m20220101_000001_create_schemas::Schemas;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum((Schemas::FinalFantasy, Alias::new("housing_type")))
                    .values(HousingType::iter().collect::<Vec<HousingType>>())
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table((Schemas::FinalFantasy, CharacterHousing::Table))
                    .add_column(
                        ColumnDef::new(CharacterHousing::HousingType)
                            .custom(Alias::new("final_fantasy.housing_type"))
                            .not_null()
                            .default(SimpleExpr::Custom("private".into())),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table((Schemas::FinalFantasy, CharacterHousing::Table))
                    .drop_column(CharacterHousing::HousingType)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum CharacterHousing {
    Table,
    HousingType,
}

#[derive(Iden, EnumIter)]
enum HousingType {
    Private,
    FreeCompany,
    SharedApartment,
}

use sea_orm_migration::prelude::*;
use crate::extension::postgres::Type;
use crate::m20220101_000001_create_schemas::Schemas;
use crate::sea_orm::{EnumIter};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_type(
                Type::alter()
                    .name((Schemas::FinalFantasy, Alias::new("fighter_job")))
                    .add_value(FighterJob::Viper)
            ).await?;
        manager
            .alter_type(
                Type::alter()
                    .name((Schemas::FinalFantasy, Alias::new("fighter_job")))
                    .add_value(FighterJob::Pictomancer)
            )
            .await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

#[derive(Iden, EnumIter)]
enum FighterJob {
    Viper,
    Pictomancer,
}

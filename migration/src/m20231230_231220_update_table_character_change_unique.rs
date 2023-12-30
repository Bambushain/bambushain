use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_schemas::Schemas;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .table((Schemas::FinalFantasy, Character::Table))
                    .col(Character::World)
                    .col(Character::Name)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .table((Schemas::FinalFantasy, Character::Table))
                    .name("character_world_name_idx")
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Character {
    Table,
    Name,
    World,
}

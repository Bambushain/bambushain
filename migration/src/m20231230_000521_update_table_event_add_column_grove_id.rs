use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_schemas::Schemas;
use crate::m20231229_235511_create_table_grove::Grove;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table((Schemas::Bamboo, Event::Table))
                    .add_column(ColumnDef::new(Event::GroveId).integer().null())
                    .add_foreign_key(
                        ForeignKey::create()
                            .from((Schemas::Bamboo, Event::Table), Event::GroveId)
                            .to((Schemas::Grove, Grove::Table), Grove::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .get_foreign_key(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table((Schemas::Bamboo, Event::Table))
                    .drop_column(Event::GroveId)
                    .drop_foreign_key(Alias::new("event_grove_grove_id_fkey"))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Event {
    Table,
    GroveId,
}

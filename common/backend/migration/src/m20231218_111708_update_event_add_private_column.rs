use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_schemas::Schemas;
use crate::m20230724_121011_create_table_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table((Schemas::Bamboo, Event::Table))
                    .add_column(
                        ColumnDef::new(Event::IsPrivate)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .add_column(ColumnDef::new(Event::UserId).integer().null())
                    .add_foreign_key(
                        ForeignKey::create()
                            .from((Schemas::Bamboo, Event::Table), Event::UserId)
                            .to((Schemas::Authentication, User::Table), User::Id)
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
                    .drop_column(Event::UserId)
                    .drop_column(Event::IsPrivate)
                    .drop_foreign_key(Alias::new("event_user_user_id_fkey"))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Event {
    Table,
    UserId,
    IsPrivate,
}

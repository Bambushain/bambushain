use crate::m20220101_000001_create_schemas::Schemas;
use crate::m20231229_235511_create_table_grove::Grove;
use crate::sea_orm::Statement;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table((Schemas::Grove, Grove::Table))
                    .drop_column(Grove::IsSuspended)
                    .drop_column(Grove::IsEnabled)
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table((Schemas::Grove, GroveUser::Table))
                    .col(ColumnDef::new(GroveUser::GroveId).integer())
                    .col(ColumnDef::new(GroveUser::UserId).integer())
                    .col(ColumnDef::new(GroveUser::IsMod).boolean().default(false))
                    .primary_key(
                        Index::create()
                            .col(GroveUser::UserId)
                            .col(GroveUser::GroveId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from((Schemas::Grove, GroveUser::Table), GroveUser::GroveId)
                            .to((Schemas::Grove, Grove::Table), Grove::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .name("grove_user_grove_id_fkey"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from((Schemas::Grove, GroveUser::Table), GroveUser::UserId)
                            .to((Schemas::Authentication, User::Table), User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .name("grove_user_user_id_fkey"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(manager.get_database_backend(), "insert into grove.grove_user select grove_id, id as user_id, is_mod from authentication.\"user\""))
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table((Schemas::Authentication, User::Table))
                    .drop_column(User::GroveId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        panic!(
            "This migration doesn't support migrating down since it entirely rewrites the database"
        )
    }
}

#[derive(Iden)]
enum GroveUser {
    Table,
    GroveId,
    UserId,
    IsMod,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
    GroveId,
}

use sea_orm_migration::prelude::*;
use crate::m20230724_165759_create_table_event::Event;
use crate::m20230724_171039_create_table_kill::Kill;
use crate::m20230724_171437_create_table_kill_to_user::KillToUser;
use crate::m20230724_171715_create_table_mount::Mount;
use crate::m20230724_171801_create_table_mount_to_user::MountToUser;
use crate::m20230724_171834_create_table_savage_mount::SavageMount;
use crate::m20230724_171907_create_table_savage_mount_to_user::SavageMountToUser;
use crate::m20230725_092805_create_table_status::Status;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(Event::Table)
                    .to_owned()
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(KillToUser::Table)
                    .to_owned()
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(Kill::Table)
                    .to_owned()
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(MountToUser::Table)
                    .to_owned()
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(Mount::Table)
                    .to_owned()
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(SavageMountToUser::Table)
                    .to_owned()
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(SavageMount::Table)
                    .to_owned()
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(Status::Table)
                    .to_owned()
            )
            .await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

use crate::m20220101_000001_create_schemas::Schemas;
use crate::m20230724_121011_create_table_user::User;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Schemas::FinalFantasy, FreeCompany::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FreeCompany::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(FreeCompany::Name).string().not_null())
                    .col(ColumnDef::new(FreeCompany::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                (Schemas::FinalFantasy, FreeCompany::Table),
                                FreeCompany::UserId,
                            )
                            .to((Schemas::Authentication, User::Table), User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .table((Schemas::FinalFantasy, FreeCompany::Table))
                            .col(FreeCompany::UserId)
                            .col(FreeCompany::Name)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table((Schemas::FinalFantasy, Character::Table))
                    .add_column(ColumnDef::new(Character::FreeCompanyId).integer().null())
                    .add_foreign_key(
                        ForeignKey::create()
                            .from(
                                (Schemas::FinalFantasy, Character::Table),
                                Character::FreeCompanyId,
                            )
                            .to((Schemas::FinalFantasy, FreeCompany::Table), FreeCompany::Id)
                            .on_delete(ForeignKeyAction::SetNull)
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
                    .table((Schemas::FinalFantasy, Character::Table))
                    .drop_foreign_key(Alias::new("character_free_company_free_company_id_fkey"))
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table((Schemas::FinalFantasy, FreeCompany::Table))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum FreeCompany {
    Table,
    Id,
    Name,
    UserId,
}

#[derive(Iden)]
enum Character {
    Table,
    FreeCompanyId,
}

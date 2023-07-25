use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_table_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Token::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Token::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Token::UserId).integer().not_null())
                    .col(ColumnDef::new(Token::Token).string().not_null().unique_key())
                    .foreign_key(ForeignKey::create()
                        .from(Token::Table, Token::UserId)
                        .to(User::Table, User::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Token::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Token {
    Table,
    Id,
    UserId,
    Token,
}
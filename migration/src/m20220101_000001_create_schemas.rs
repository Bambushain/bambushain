use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let pandaparty_stmt =
            Statement::from_string(manager.get_database_backend(), "CREATE SCHEMA panda_party");
        let finalfantasy_stmt = Statement::from_string(
            manager.get_database_backend(),
            "CREATE SCHEMA final_fantasy",
        );
        let authentication_stmt = Statement::from_string(
            manager.get_database_backend(),
            "CREATE SCHEMA authentication",
        );

        db.execute(pandaparty_stmt).await?;
        db.execute(finalfantasy_stmt).await?;
        db.execute(authentication_stmt).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let panda_party_stmt =
            Statement::from_string(manager.get_database_backend(), "DROP SCHEMA panda_party");
        let final_fantasy_stmt =
            Statement::from_string(manager.get_database_backend(), "DROP SCHEMA final_fantasy");
        let authentication_stmt =
            Statement::from_string(manager.get_database_backend(), "DROP SCHEMA authentication");

        db.execute(panda_party_stmt).await?;
        db.execute(final_fantasy_stmt).await?;
        db.execute(authentication_stmt).await?;

        Ok(())
    }
}

#[derive(Iden)]
pub enum Schemas {
    PandaParty,
    FinalFantasy,
    Authentication,
}

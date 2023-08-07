pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table_user;
mod m20230724_165124_create_table_token;
mod m20230724_165521_create_table_crafter;
mod m20230724_165656_create_table_fighter;
mod m20230724_165759_create_table_event;
mod m20230724_171039_create_table_kill;
mod m20230724_171437_create_table_kill_to_user;
mod m20230724_171715_create_table_mount;
mod m20230724_171801_create_table_mount_to_user;
mod m20230724_171834_create_table_savage_mount;
mod m20230724_171907_create_table_savage_mount_to_user;
mod m20230725_092805_create_table_status;
mod m20230804_191618_delete_old_tables;
mod m20230805_202224_user_add_discord_name;
mod m20230805_211912_create_table_calendar;
mod m20230807_004619_update_table_fighter;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table_user::Migration),
            Box::new(m20230724_165124_create_table_token::Migration),
            Box::new(m20230724_165521_create_table_crafter::Migration),
            Box::new(m20230724_165656_create_table_fighter::Migration),
            Box::new(m20230724_165759_create_table_event::Migration),
            Box::new(m20230724_171039_create_table_kill::Migration),
            Box::new(m20230724_171437_create_table_kill_to_user::Migration),
            Box::new(m20230724_171715_create_table_mount::Migration),
            Box::new(m20230724_171801_create_table_mount_to_user::Migration),
            Box::new(m20230724_171834_create_table_savage_mount::Migration),
            Box::new(m20230724_171907_create_table_savage_mount_to_user::Migration),
            Box::new(m20230725_092805_create_table_status::Migration),
            Box::new(m20230804_191618_delete_old_tables::Migration),
            Box::new(m20230805_202224_user_add_discord_name::Migration),
            Box::new(m20230805_211912_create_table_calendar::Migration),
            Box::new(m20230807_004619_update_table_fighter::Migration),
        ]
    }
}

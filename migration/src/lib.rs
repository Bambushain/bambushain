pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_schemas;
mod m20230724_121011_create_table_user;
mod m20230724_121111_create_table_character;
mod m20230724_165124_create_table_token;
mod m20230724_165521_create_table_crafter;
mod m20230724_165656_create_table_fighter;
mod m20230724_165759_create_table_event;
mod m20230826_221916_update_user_add_otp_column;
mod m20230829_194031_create_table_custom_character_field;
mod m20230829_194055_create_table_custom_character_field_option;
mod m20230829_194101_create_table_custom_character_field_value;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_schemas::Migration),
            Box::new(m20230724_121011_create_table_user::Migration),
            Box::new(m20230724_121111_create_table_character::Migration),
            Box::new(m20230724_165124_create_table_token::Migration),
            Box::new(m20230724_165521_create_table_crafter::Migration),
            Box::new(m20230724_165656_create_table_fighter::Migration),
            Box::new(m20230724_165759_create_table_event::Migration),
            Box::new(m20230826_221916_update_user_add_otp_column::Migration),
            Box::new(m20230829_194031_create_table_custom_character_field::Migration),
            Box::new(m20230829_194055_create_table_custom_character_field_option::Migration),
            Box::new(m20230829_194101_create_table_custom_character_field_value::Migration),
        ]
    }
}

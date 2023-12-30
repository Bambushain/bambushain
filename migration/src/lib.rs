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
mod m20230917_003256_add_position_to_custom_field;
mod m20230923_090306_add_field_free_company;
mod m20231128_215928_rename_schema_panda_party;
mod m20231129_222204_add_field_to_set_totp_secret_encrypted;
mod m20231130_013324_increase_size_of_two_factor_code;
mod m20231212_004733_create_table_character_housing;
mod m20231218_111708_update_event_add_private_column;
mod m20231223_002207_update_housing_add_column_type;
mod m20231229_235511_create_table_grove;
mod m20231230_000521_update_table_event_add_column_grove_id;
mod m20231230_001307_update_table_user_add_column_grove_id;

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
            Box::new(m20230917_003256_add_position_to_custom_field::Migration),
            Box::new(m20230923_090306_add_field_free_company::Migration),
            Box::new(m20231128_215928_rename_schema_panda_party::Migration),
            Box::new(m20231129_222204_add_field_to_set_totp_secret_encrypted::Migration),
            Box::new(m20231130_013324_increase_size_of_two_factor_code::Migration),
            Box::new(m20231212_004733_create_table_character_housing::Migration),
            Box::new(m20231218_111708_update_event_add_private_column::Migration),
            Box::new(m20231223_002207_update_housing_add_column_type::Migration),
            Box::new(m20231229_235511_create_table_grove::Migration),
            Box::new(m20231230_000521_update_table_event_add_column_grove_id::Migration),
            Box::new(m20231230_001307_update_table_user_add_column_grove_id::Migration),
        ]
    }
}

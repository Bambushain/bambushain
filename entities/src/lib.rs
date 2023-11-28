pub mod authentication;
pub mod character;
pub mod crafter;
pub mod custom_character_field;
pub mod custom_character_field_option;
pub mod custom_character_field_value;
pub mod error;
pub mod event;
pub mod fighter;
pub mod free_company;
pub mod result;
pub mod token;
pub mod user;

pub mod prelude {
    pub use crate::authentication::*;
    pub use crate::character::CharacterRace;
    pub use crate::character::Model as Character;
    pub use crate::crafter::CrafterJob;
    pub use crate::crafter::Model as Crafter;
    pub use crate::custom_character_field::CustomField;
    pub use crate::custom_character_field::Model as CustomCharacterField;
    pub use crate::custom_character_field_option::Model as CustomCharacterFieldOption;
    pub use crate::custom_character_field_value::Model as CustomCharacterFieldValue;
    pub use crate::error::*;
    pub use crate::event::Model as Event;
    pub use crate::fighter::FighterJob;
    pub use crate::fighter::Model as Fighter;
    pub use crate::free_company::Model as FreeCompany;
    pub use crate::result::*;
    pub use crate::token::Model as Token;
    pub use crate::user::Model as User;
    pub use crate::user::TotpQrCode;
    pub use crate::user::UpdateProfile;
    pub use crate::user::ValidateTotp;
    pub use crate::user::WebUser;
    pub use crate::{
        bamboo_db_error, bamboo_exists_already_error, bamboo_insufficient_rights_error,
        bamboo_invalid_data_error, bamboo_not_found_error, bamboo_unknown_error,
        bamboo_validation_error,
    };
}

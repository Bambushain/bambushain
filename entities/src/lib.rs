pub mod crafter;
pub mod event;
pub mod fighter;
pub mod kill;
#[cfg(feature = "backend")]
pub mod kill_to_user;
pub mod mount;
#[cfg(feature = "backend")]
pub mod mount_to_user;
pub mod savage_mount;
#[cfg(feature = "backend")]
pub mod savage_mount_to_user;
pub mod token;
pub mod user;
pub mod error;
pub mod result;
pub mod authentication;
pub mod calendar;

pub mod prelude {
    pub use crate::{sheef_exists_already_error, sheef_insufficient_rights_error, sheef_invalid_data_error, sheef_not_found_error, sheef_unknown_error, sheef_validation_error};
    pub use crate::authentication::*;
    pub use crate::calendar::*;
    pub use crate::crafter::Model as Crafter;
    pub use crate::error::*;
    pub use crate::event::Model as Event;
    pub use crate::fighter::Model as Fighter;
    pub use crate::kill::Model as Kill;
    #[cfg(feature = "backend")]
    pub use crate::kill_to_user::Model as KillToUser;
    pub use crate::mount::Model as Mount;
    #[cfg(feature = "backend")]
    pub use crate::mount_to_user::Model as MountToUser;
    pub use crate::result::*;
    pub use crate::savage_mount::Model as SavageMount;
    #[cfg(feature = "backend")]
    pub use crate::savage_mount_to_user::Model as SavageMountToUser;
    pub use crate::token::Model as Token;
    pub use crate::user::Model as User;
    pub use crate::user::UpdateProfile;
    pub use crate::user::WebUser;
}
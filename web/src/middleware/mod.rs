macro_rules! box_pin {
    ($a:expr) => {
        Box::pin(async move {$a})
    };
}

pub mod authenticate_user;
pub mod check_mod;
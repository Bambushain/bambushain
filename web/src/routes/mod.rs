macro_rules! username {
    ($req:ident) => {
        {
            use crate::middleware::authenticate_user::AuthenticationState;
            use actix_web::HttpMessage;

            let u = {
                let extensions = $req.extensions();
                let state = extensions.get::<AuthenticationState>().expect("AuthenticationState should be set");
                state.user.username.to_string()
            };

            u
        }
    };
}

pub mod user;
pub mod authentication;
pub mod crafter;
pub mod fighter;
pub mod calendar;
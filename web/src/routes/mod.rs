macro_rules! username {
    ($req:ident) => {
        {
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
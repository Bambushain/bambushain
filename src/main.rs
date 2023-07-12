use actix_web::{App, HttpServer};
use actix_web::web::{delete, get, post, put};
use log::info;
use crate::web::middleware::authenticate_user::AuthenticateUser;
use crate::web::middleware::check_mod::CheckMod;
use crate::web::routes::user::{add_main_group_user, add_mod_user, create_user, delete_user, get_user, get_users, remove_main_group_user, remove_mod_user};

mod web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    stderrlog::new()
        .module(module_path!())
        .verbosity(log::Level::Trace)
        .init()
        .unwrap();

    info!("Running sheef_planing on :8070");

    HttpServer::new(|| {
        App::new()
            .route("/api/user/{username}", get().to(get_user).wrap(AuthenticateUser))
            .route("/api/user/{username}", delete().to(delete_user).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user/{username}/mod", put().to(add_mod_user).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user/{username}/mod", delete().to(remove_mod_user).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user/{username}/main", put().to(add_main_group_user).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user/{username}/main", delete().to(remove_main_group_user).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user", get().to(get_users).wrap(AuthenticateUser))
            .route("/api/user", post().to(create_user).wrap(CheckMod).wrap(AuthenticateUser))

    })
        .bind(("0.0.0.0", 8070))?
        .run()
        .await
}
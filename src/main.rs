use actix_web::{App, HttpServer};
use actix_web::web::{delete, get, post, put};
use log::info;
use sheef_web::middleware::authenticate_user::AuthenticateUser;
use sheef_web::middleware::check_mod::CheckMod;
use sheef_web::routes::authentication::{login, logout};
use sheef_web::routes::calendar::{get_calendar, get_day_details, update_day_details};
use sheef_web::routes::crafter::{create_crafter, delete_crafter, get_crafter, get_crafters, update_crafter};
use sheef_web::routes::fighter::{create_fighter, delete_fighter, get_fighter, get_fighters, update_fighter};
use sheef_web::routes::user::{add_main_group_user, add_mod_user, change_password, create_user, delete_user, get_user, get_users, remove_main_group_user, remove_mod_user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    stderrlog::new()
        .module(module_path!())
        .verbosity(log::Level::Trace)
        .init()
        .unwrap();

    info!("Running sheef planing on :8070");

    HttpServer::new(|| {
        App::new()
            .route("/api/login", post().to(login))
            .route("/api/login", delete().to(logout).wrap(AuthenticateUser))

            .route("/api/user", get().to(get_users).wrap(AuthenticateUser))
            .route("/api/user", post().to(create_user).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user/{username}", get().to(get_user).wrap(AuthenticateUser))
            .route("/api/user/{username}", delete().to(delete_user).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user/{username}/mod", put().to(add_mod_user).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user/{username}/mod", delete().to(remove_mod_user).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user/{username}/main", put().to(add_main_group_user).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user/{username}/main", delete().to(remove_main_group_user).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user/{username}/password", put().to(change_password).wrap(CheckMod).wrap(AuthenticateUser))

            .route("/api/crafter", get().to(get_crafters).wrap(AuthenticateUser))
            .route("/api/crafter", post().to(create_crafter).wrap(AuthenticateUser))
            .route("/api/crafter/{job}", get().to(get_crafter).wrap(AuthenticateUser))
            .route("/api/crafter/{job}", put().to(update_crafter).wrap(AuthenticateUser))
            .route("/api/crafter/{job}", delete().to(delete_crafter).wrap(AuthenticateUser))

            .route("/api/fighter", get().to(get_fighters).wrap(AuthenticateUser))
            .route("/api/fighter", post().to(create_fighter).wrap(AuthenticateUser))
            .route("/api/fighter/{job}", get().to(get_fighter).wrap(AuthenticateUser))
            .route("/api/fighter/{job}", put().to(update_fighter).wrap(AuthenticateUser))
            .route("/api/fighter/{job}", delete().to(delete_fighter).wrap(AuthenticateUser))

            .route("/api/calendar", get().to(get_calendar).wrap(AuthenticateUser))
            .route("/api/calendar/{year}/{month}/{day}", get().to(get_day_details).wrap(AuthenticateUser))
            .route("/api/calendar/{year}/{month}/{day}", put().to(update_day_details).wrap(AuthenticateUser))
    })
        .bind(("0.0.0.0", 8070))?
        .run()
        .await
}

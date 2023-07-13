use actix_web::{App, HttpServer};
use actix_web::web::{delete, get, post, put};
use log::info;
use sheef_web::middleware::authenticate_user::AuthenticateUser;
use sheef_web::middleware::check_mod::CheckMod;
use sheef_web::routes::authentication::{login, logout};
use sheef_web::routes::calendar::{get_calendar, get_day_details, update_day_details};
use sheef_web::routes::crafter::{create_crafter, delete_crafter, get_crafter, get_crafters, update_crafter};
use sheef_web::routes::fighter::{create_fighter, delete_fighter, get_fighter, get_fighters, update_fighter};
use sheef_web::routes::kill::{activate_kill_for_me, activate_kill_for_user, create_kill, deactivate_kill_for_me, deactivate_kill_for_user, delete_kill, get_kills, get_kills_for_user, get_my_kills, get_users_for_kill, update_kill};
use sheef_web::routes::savage_mount::{activate_savage_mount_for_me, activate_savage_mount_for_user, create_savage_mount, deactivate_savage_mount_for_me, deactivate_savage_mount_for_user, delete_savage_mount, get_savage_mounts, get_savage_mounts_for_user, get_my_savage_mounts, get_users_for_savage_mount, update_savage_mount};
use sheef_web::routes::mount::{activate_mount_for_me, activate_mount_for_user, create_mount, deactivate_mount_for_me, deactivate_mount_for_user, delete_mount, get_mounts, get_mounts_for_user, get_my_mounts, get_users_for_mount, update_mount};
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

            .route("/api/user/{username}/kill", get().to(get_kills_for_user).wrap(AuthenticateUser))
            .route("/api/user/{username}/kill/{kill}", put().to(activate_kill_for_user).wrap(AuthenticateUser))
            .route("/api/user/{username}/kill/{kill}", delete().to(deactivate_kill_for_user).wrap(AuthenticateUser))
            .route("/api/kill", get().to(get_kills).wrap(AuthenticateUser))
            .route("/api/kill", post().to(create_kill).wrap(AuthenticateUser))
            .route("/api/kill/{kill}", get().to(get_users_for_kill).wrap(AuthenticateUser))
            .route("/api/kill/{kill}", put().to(update_kill).wrap(AuthenticateUser))
            .route("/api/kill/{kill}", delete().to(delete_kill).wrap(AuthenticateUser))
            .route("/api/my/kill", get().to(get_my_kills).wrap(AuthenticateUser))
            .route("/api/my/kill/{kill}", put().to(activate_kill_for_me).wrap(AuthenticateUser))
            .route("/api/my/kill/{kill}", delete().to(deactivate_kill_for_me).wrap(AuthenticateUser))

            .route("/api/user/{username}/mount", get().to(get_mounts_for_user).wrap(AuthenticateUser))
            .route("/api/user/{username}/mount/{mount}", put().to(activate_mount_for_user).wrap(AuthenticateUser))
            .route("/api/user/{username}/mount/{mount}", delete().to(deactivate_mount_for_user).wrap(AuthenticateUser))
            .route("/api/mount", get().to(get_mounts).wrap(AuthenticateUser))
            .route("/api/mount", post().to(create_mount).wrap(AuthenticateUser))
            .route("/api/mount/{mount}", get().to(get_users_for_mount).wrap(AuthenticateUser))
            .route("/api/mount/{mount}", put().to(update_mount).wrap(AuthenticateUser))
            .route("/api/mount/{mount}", delete().to(delete_mount).wrap(AuthenticateUser))
            .route("/api/my/mount", get().to(get_my_mounts).wrap(AuthenticateUser))
            .route("/api/my/mount/{mount}", put().to(activate_mount_for_me).wrap(AuthenticateUser))
            .route("/api/my/mount/{mount}", delete().to(deactivate_mount_for_me).wrap(AuthenticateUser))

            .route("/api/user/{username}/savage-mount", get().to(get_savage_mounts_for_user).wrap(AuthenticateUser))
            .route("/api/user/{username}/savage-mount/{savage_mount}", put().to(activate_savage_mount_for_user).wrap(AuthenticateUser))
            .route("/api/user/{username}/savage-mount/{savage_mount}", delete().to(deactivate_savage_mount_for_user).wrap(AuthenticateUser))
            .route("/api/savage-mount", get().to(get_savage_mounts).wrap(AuthenticateUser))
            .route("/api/savage-mount", post().to(create_savage_mount).wrap(AuthenticateUser))
            .route("/api/savage-mount/{savage_mount}", get().to(get_users_for_savage_mount).wrap(AuthenticateUser))
            .route("/api/savage-mount/{savage_mount}", put().to(update_savage_mount).wrap(AuthenticateUser))
            .route("/api/savage-mount/{savage_mount}", delete().to(delete_savage_mount).wrap(AuthenticateUser))
            .route("/api/my/savage-mount", get().to(get_my_savage_mounts).wrap(AuthenticateUser))
            .route("/api/my/savage-mount/{savage_mount}", put().to(activate_savage_mount_for_me).wrap(AuthenticateUser))
            .route("/api/my/savage-mount/{savage_mount}", delete().to(deactivate_savage_mount_for_me).wrap(AuthenticateUser))
    })
        .bind(("0.0.0.0", 8070))?
        .run()
        .await
}

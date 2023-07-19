use std::sync::Arc;

use actix_web::{App, guard, HttpResponse, HttpServer};
use actix_web::web;

use sheef_backend::broadcaster::calendar::CalendarBroadcaster;
use sheef_backend::broadcaster::crew::CrewBroadcaster;
use sheef_backend::broadcaster::kill::KillBroadcaster;
use sheef_backend::broadcaster::mount::MountBroadcaster;
use sheef_backend::broadcaster::savage_mount::SavageMountBroadcaster;
use sheef_backend::middleware::authenticate_user::AuthenticateUser;
use sheef_backend::middleware::check_mod::CheckMod;
use sheef_backend::routes::authentication::{login, logout};
use sheef_backend::routes::calendar::{get_calendar, get_day_details, update_day_details};
use sheef_backend::routes::crafter::{create_crafter, delete_crafter, get_crafter, get_crafters, update_crafter};
use sheef_backend::routes::fighter::{create_fighter, delete_fighter, get_fighter, get_fighters, update_fighter};
use sheef_backend::routes::kill::{activate_kill_for_me, activate_kill_for_user, create_kill, deactivate_kill_for_me, deactivate_kill_for_user, delete_kill, get_kills, update_kill};
use sheef_backend::routes::mount::{activate_mount_for_me, activate_mount_for_user, create_mount, deactivate_mount_for_me, deactivate_mount_for_user, delete_mount, get_mounts, update_mount};
use sheef_backend::routes::savage_mount::{activate_savage_mount_for_me, activate_savage_mount_for_user, create_savage_mount, deactivate_savage_mount_for_me, deactivate_savage_mount_for_user, delete_savage_mount, get_savage_mounts, update_savage_mount};
use sheef_backend::routes::user::{add_main_group_user, add_mod_user, change_my_password, change_password, create_user, delete_user, get_profile, get_user, get_users, remove_main_group_user, remove_mod_user, update_profile, update_user_profile};
use sheef_backend::sse::calendar::calendar_sse_client;
use sheef_backend::sse::crew::crew_sse_client;
use sheef_backend::sse::kill::kill_sse_client;
use sheef_backend::sse::mount::mount_sse_client;
use sheef_backend::sse::NotificationState;
use sheef_backend::sse::savage_mount::savage_mount_sse_client;

macro_rules! static_file_str {
    ($file:expr, $content_type:expr, $fn_name:tt) => {
        async fn $fn_name() -> actix_web::HttpResponse {
            actix_web::HttpResponse::Ok().content_type($content_type).body(include_str!($file))
        }
    };
}

macro_rules! static_file_bytes {
    ($file:expr, $content_type:expr, $fn_name:tt) => {
        async fn $fn_name() -> actix_web::HttpResponse {
            actix_web::HttpResponse::Ok().content_type($content_type).body(actix_web::web::Bytes::from_static(include_bytes!($file).as_slice()))
        }
    };
}

static_file_str!("../rusty/dist/custom.css", "text/css", custom_css);
static_file_str!("../rusty/dist/pico.css", "text/css", pico_css);
static_file_str!("../rusty/dist/rusty_sheef.js", "application/javascript", rusty_sheef_js);
static_file_str!("../rusty/dist/index.html", "text/html", index_html);
static_file_bytes!("../rusty/dist/rusty_sheef_bg.wasm", "application/wasm", rusty_sheef_bg_wasm);
static_file_bytes!("../rusty/dist/favicon.png", "image/png", favicon_png);
static_file_bytes!("../rusty/dist/login.png", "image/png", login_png);

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    stderrlog::new()
        .verbosity(log::Level::Info)
        .init()
        .unwrap();

    log::info!("Running sheef planing on :8070");

    let calendar_broadcaster = CalendarBroadcaster::create();
    let kill_broadcaster = KillBroadcaster::create();
    let mount_broadcaster = MountBroadcaster::create();
    let savage_mount_broadcaster = SavageMountBroadcaster::create();
    let crew_broadcaster = CrewBroadcaster::create();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(NotificationState {
                calendar_broadcaster: Arc::clone(&calendar_broadcaster),
                kill_broadcaster: Arc::clone(&kill_broadcaster),
                mount_broadcaster: Arc::clone(&mount_broadcaster),
                savage_mount_broadcaster: Arc::clone(&savage_mount_broadcaster),
                crew_broadcaster: Arc::clone(&crew_broadcaster),
            }))

            .route("/api/login", web::post().to(login))
            .route("/api/login", web::delete().to(logout).wrap(AuthenticateUser))
            .route("/api/login", web::head().to(HttpResponse::NoContent).wrap(AuthenticateUser))

            .route("/api/user", web::get().to(get_users).wrap(AuthenticateUser))
            .route("/api/user", web::post().to(create_user).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user/{username}", web::get().to(get_user).wrap(AuthenticateUser))
            .route("/api/user/{username}", web::delete().to(delete_user).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user/{username}/profile", web::put().to(update_user_profile).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user/{username}/mod", web::put().to(add_mod_user).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user/{username}/mod", web::delete().to(remove_mod_user).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user/{username}/main", web::put().to(add_main_group_user).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user/{username}/main", web::delete().to(remove_main_group_user).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user/{username}/password", web::put().to(change_password).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user/{username}/kill/{kill}", web::put().to(activate_kill_for_user).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user/{username}/kill/{kill}", web::delete().to(deactivate_kill_for_user).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user/{username}/mount/{mount}", web::put().to(activate_mount_for_user).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user/{username}/mount/{mount}", web::delete().to(deactivate_mount_for_user).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user/{username}/savage-mount/{savage_mount}", web::put().to(activate_savage_mount_for_user).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/user/{username}/savage-mount/{savage_mount}", web::delete().to(deactivate_savage_mount_for_user).wrap(CheckMod).wrap(AuthenticateUser))

            .route("/api/crafter", web::get().to(get_crafters).wrap(AuthenticateUser))
            .route("/api/crafter", web::post().to(create_crafter).wrap(AuthenticateUser))
            .route("/api/crafter/{job}", web::get().to(get_crafter).wrap(AuthenticateUser))
            .route("/api/crafter/{job}", web::put().to(update_crafter).wrap(AuthenticateUser))
            .route("/api/crafter/{job}", web::delete().to(delete_crafter).wrap(AuthenticateUser))

            .route("/api/fighter", web::get().to(get_fighters).wrap(AuthenticateUser))
            .route("/api/fighter", web::post().to(create_fighter).wrap(AuthenticateUser))
            .route("/api/fighter/{job}", web::get().to(get_fighter).wrap(AuthenticateUser))
            .route("/api/fighter/{job}", web::put().to(update_fighter).wrap(AuthenticateUser))
            .route("/api/fighter/{job}", web::delete().to(delete_fighter).wrap(AuthenticateUser))

            .route("/api/calendar", web::get().to(get_calendar).wrap(AuthenticateUser))
            .route("/api/calendar/{year}/{month}/{day}", web::get().to(get_day_details).wrap(AuthenticateUser))
            .route("/api/calendar/{year}/{month}/{day}", web::put().to(update_day_details).wrap(AuthenticateUser))

            .route("/api/kill", web::get().to(get_kills).wrap(AuthenticateUser))
            .route("/api/kill", web::post().to(create_kill).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/kill/{kill}", web::put().to(update_kill).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/kill/{kill}", web::delete().to(delete_kill).wrap(CheckMod).wrap(AuthenticateUser))

            .route("/api/mount", web::get().to(get_mounts).wrap(AuthenticateUser))
            .route("/api/mount", web::post().to(create_mount).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/mount/{mount}", web::put().to(update_mount).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/mount/{mount}", web::delete().to(delete_mount).wrap(CheckMod).wrap(AuthenticateUser))

            .route("/api/savage-mount", web::get().to(get_savage_mounts).wrap(AuthenticateUser))
            .route("/api/savage-mount", web::post().to(create_savage_mount).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/savage-mount/{savage_mount}", web::put().to(update_savage_mount).wrap(CheckMod).wrap(AuthenticateUser))
            .route("/api/savage-mount/{savage_mount}", web::delete().to(delete_savage_mount).wrap(CheckMod).wrap(AuthenticateUser))

            .route("/api/my/profile", web::get().to(get_profile).wrap(AuthenticateUser))
            .route("/api/my/profile", web::put().to(update_profile).wrap(AuthenticateUser))
            .route("/api/my/password", web::put().to(change_my_password).wrap(AuthenticateUser))
            .route("/api/my/kill/{kill}", web::put().to(activate_kill_for_me).wrap(AuthenticateUser))
            .route("/api/my/kill/{kill}", web::delete().to(deactivate_kill_for_me).wrap(AuthenticateUser))
            .route("/api/my/mount/{mount}", web::put().to(activate_mount_for_me).wrap(AuthenticateUser))
            .route("/api/my/mount/{mount}", web::delete().to(deactivate_mount_for_me).wrap(AuthenticateUser))
            .route("/api/my/savage-mount/{savage_mount}", web::put().to(activate_savage_mount_for_me).wrap(AuthenticateUser))
            .route("/api/my/savage-mount/{savage_mount}", web::delete().to(deactivate_savage_mount_for_me).wrap(AuthenticateUser))

            .route("/sse/calendar", web::get().to(calendar_sse_client))
            .route("/sse/crew", web::get().to(crew_sse_client))
            .route("/sse/kill", web::get().to(kill_sse_client))
            .route("/sse/mount", web::get().to(mount_sse_client))
            .route("/sse/savage-mount", web::get().to(savage_mount_sse_client))

            .route("/static/custom.css", web::get().to(custom_css))
            .route("/static/pico.css", web::get().to(pico_css))
            .route("/static/rusty_sheef.js", web::get().to(rusty_sheef_js))
            .route("/static/rusty_sheef_bg.wasm", web::get().to(rusty_sheef_bg_wasm))
            .route("/static/favicon.png", web::get().to(favicon_png))
            .route("/static/login.png", web::get().to(login_png))

            .default_service(web::route().guard(guard::Get()).to(index_html))
    })
        .bind(("0.0.0.0", 8070))?
        .run()
        .await
}

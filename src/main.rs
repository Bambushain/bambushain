use std::sync::Arc;

use actix_web::web;
use actix_web::{App, HttpResponse, HttpServer};
use rand::distributions::Alphanumeric;
use rand::prelude::*;
use sea_orm::prelude::*;

use bamboo_backend::{DbConnection, Services, ServicesState};
use bamboo_entities::user;
use bamboo_migration::{IntoSchemaManagerConnection, Migrator, MigratorTrait};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    stderrlog::new().verbosity(log::Level::Info).init().unwrap();

    log::info!("Open the bamboo grove");

    let mut opts =
        sea_orm::ConnectOptions::new(std::env::var("DATABASE_URL").expect("Needs DATABASE_URL"));
    opts.sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Debug);

    let db = match sea_orm::Database::connect(opts).await {
        Ok(db) => db,
        Err(err) => panic!("{err}"),
    };

    match Migrator::up(db.into_schema_manager_connection(), None).await {
        Ok(_) => log::info!("Successfully migrated database"),
        Err(err) => panic!("{err}"),
    }

    let at_least_one_mod_exists = match user::Entity::find()
        .filter(user::Column::IsMod.eq(true))
        .count(&db)
        .await
    {
        Ok(count) => {
            log::info!("Database contains {count} users");
            count > 0
        }
        Err(err) => panic!("{err}"),
    };

    if !at_least_one_mod_exists {
        log::info!("No mod exists, creating initial user");
        let password = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(12)
            .map(char::from)
            .collect::<String>();

        let display_name = std::env::var("INITIAL_USER_DISPLAY_NAME")
            .expect("INITIAL_USER_DISPLAY_NAME must be set");
        let email = std::env::var("INITIAL_USER_EMAIL").expect("INITIAL_USER_EMAIL must be set");

        match bamboo_dbal::user::create_user(
            user::Model::new(
                email.clone(),
                password.clone(),
                display_name.clone(),
                "".into(),
                true,
            ),
            &db,
        )
        .await
        {
            Ok(_) => log::info!("Created initial user {email} with password {password}"),
            Err(err) => panic!("Failed to create initial user, {err}"),
        }
    }

    log::info!("Serving on port 8070");
    HttpServer::new(move || {
        App::new()
            .app_data(DbConnection::new(db.clone()))
            .configure(bamboo_backend::routes::configure_routes)
    })
    .bind(("0.0.0.0", 8070))?
    .run()
    .await
}

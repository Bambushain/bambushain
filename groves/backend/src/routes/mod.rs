use actix_web::web;
use bamboo_common::backend::services::{EnvService, EnvironmentService};
use std::fs::File;
use uuid::Uuid;

mod groves;
mod user;

fn prepare_index_file(
    frontend_base_path: impl Into<String> + std::fmt::Display,
) -> impl Into<String> + std::fmt::Display + Clone {
    let err = "index.html must be available";

    let index_file = File::open(format!("{frontend_base_path}/dist/index.html")).expect(err);

    let mut engine = handlebars::Handlebars::new();
    let tmpl = std::io::read_to_string(index_file).expect(err);
    handlebars::handlebars_helper!(get_env: |key: String| {
        let environment_service = EnvironmentService::new();
        environment_service.get_env(key, "")
    });
    engine.register_helper("get_env", Box::new(get_env));

    let mut tmp_file_name = std::env::temp_dir();
    tmp_file_name.push(format!("{}.html", Uuid::new_v4()));

    let output = File::create(tmp_file_name.clone()).expect(err);
    engine
        .render_template_to_write(tmpl.as_str(), &None::<String>, output)
        .expect(err);

    format!("{}", tmp_file_name.display())
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    let environment_service = EnvService::new(EnvironmentService::new());

    let frontend_base_path = environment_service.get_env("FRONTEND_DIR", ".");
    log::info!("Frontend base path: {frontend_base_path}");

    log::info!("Prepare the index.html file with the environment data");
    let index_file = prepare_index_file(frontend_base_path.clone());
    log::info!("The index file is stored in {}", index_file.clone());

    cfg.app_data(environment_service)
        .service(groves::get_groves)
        .service(groves::get_grove)
        .service(groves::create_grove)
        .service(groves::suspend_grove)
        .service(groves::resume_grove)
        .service(groves::delete_grove)
        .service(user::get_users)
        .service(user::reset_user_password)
        .service(user::make_user_mod)
        .service(user::remove_user_mod)
        .service(
            actix_web_lab::web::spa()
                .index_file(index_file.into())
                .static_resources_location(format!("{frontend_base_path}/dist"))
                .static_resources_mount("/static")
                .finish(),
        );
}

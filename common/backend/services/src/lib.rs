use actix_web::web;
use sea_orm::DatabaseConnection;

pub use crate::environment_service::EnvironmentService;

mod environment_service;
pub mod minio_service;

pub type EnvService = web::Data<EnvironmentService>;
pub type DbConnection = web::Data<DatabaseConnection>;
pub type MinioService = web::Data<minio_service::MinioClient>;

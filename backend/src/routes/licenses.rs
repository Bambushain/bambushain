use actix_web::{get, HttpResponse};

#[get("/api/licenses")]
pub async fn get_licenses() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(include_str!(concat!(env!("OUT_DIR"), "/dependencies.json")))
}

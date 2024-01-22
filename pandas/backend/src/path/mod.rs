use actix_web::web;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CharacterPathInfo {
    pub character_id: i32,
}

#[derive(Deserialize)]
pub struct CrafterPathInfo {
    pub crafter_id: i32,
}

#[derive(Deserialize)]
pub struct CharacterHousingPathInfo {
    pub character_housing_id: i32,
}

#[derive(Deserialize)]
pub struct CustomFieldPathInfo {
    pub field_id: i32,
}

#[derive(Deserialize)]
pub struct CustomFieldOptionPathInfo {
    pub option_id: i32,
    pub field_id: i32,
}

#[derive(Deserialize)]
pub struct CustomFieldPositionPathInfo {
    pub field_id: i32,
    pub position: i32,
}

#[derive(Deserialize)]
pub struct EventPathInfo {
    pub event_id: i32,
}

#[derive(Deserialize)]
pub struct FighterPathInfo {
    pub fighter_id: i32,
}

#[derive(Deserialize)]
pub struct FreeCompanyPathInfo {
    pub free_company_id: i32,
}

#[derive(Deserialize)]
pub struct UserPathInfo {
    pub user_id: i32,
}

pub type CharacterPath = web::Path<CharacterPathInfo>;
pub type CharacterHousingPath = web::Path<CharacterHousingPathInfo>;
pub type CrafterPath = web::Path<CrafterPathInfo>;
pub type CustomFieldPath = web::Path<CustomFieldPathInfo>;
pub type CustomFieldOptionPath = web::Path<CustomFieldOptionPathInfo>;
pub type CustomFieldPositionPath = web::Path<CustomFieldPositionPathInfo>;
pub type EventPath = web::Path<EventPathInfo>;
pub type FighterPath = web::Path<FighterPathInfo>;
pub type FreeCompanyPath = web::Path<FreeCompanyPathInfo>;
pub type UserPath = web::Path<UserPathInfo>;

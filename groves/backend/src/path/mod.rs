use serde::Deserialize;

#[derive(Deserialize)]
pub struct GrovePath {
    pub grove_id: i32,
}

#[derive(Deserialize)]
pub struct GroveUserPath {
    pub grove_id: i32,
    pub user_id: i32,
}

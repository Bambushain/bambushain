#[derive(Default, Clone)]
pub struct EnvironmentService {}

impl EnvironmentService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_env(&self, key: impl Into<String>, default: impl Into<String>) -> String {
        std::env::var(key.into()).unwrap_or(default.into())
    }

    pub fn get_env_opt(&self, key: impl Into<String>) -> Option<String> {
        std::env::var(key.into()).ok()
    }
}

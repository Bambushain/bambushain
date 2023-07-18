#[macro_export]
macro_rules! sort_strings_insensitive {
    ($vec:expr) => {
        $vec.sort_by_key(|item| item.to_lowercase())
    };
}

pub fn get_database_base_dir() -> String {
    std::env::var("DATABASE_DIR").unwrap_or(std::env::current_dir().expect("Current dir is not available").into_os_string().to_str().unwrap().to_string())
}

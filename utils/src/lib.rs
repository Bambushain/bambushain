#[macro_export]
macro_rules! sort_strings_insensitive {
    ($vec:expr) => {
        $vec.sort_by_key(|item| item.to_lowercase())
    };
}

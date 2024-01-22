mod app;

pub use app::start_frontend;

pub mod base {
    pub use bamboo_pandas_frontend_base::*;
}

pub mod sections {
    pub use bamboo_pandas_frontend_sections::*;
}
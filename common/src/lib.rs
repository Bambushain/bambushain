#[cfg(feature = "core")]
pub use bamboo_common_core as core;

#[cfg(feature = "frontend")]
pub mod frontend {
    pub use bamboo_common_frontend::ui;
}

#[cfg(feature = "backend")]
pub mod backend {
    pub use bamboo_common_backend::dbal;
    pub use bamboo_common_backend::services;
}


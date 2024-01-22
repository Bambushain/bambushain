#[cfg(feature = "frontend")]
pub mod frontend {
    pub use bamboo_common_frontend::ui as ui;
}

#[cfg(feature = "backend")]
pub mod backend {
    pub use bamboo_common_backend::dbal as dbal;
    pub use bamboo_common_backend::services as services;
}

#[cfg(feature = "core")]
pub mod core {
    pub use bamboo_common_core::*;
}
use serde::{Deserialize, Serialize};

use bamboo_common_core_error::BambooError;

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct SupportRequest {
    pub subject: String,
    pub message: String,
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct GlitchTipErrorRequest {
    pub page: String,
    pub form: String,
    pub bamboo_error: BambooError,
    pub full_url: String,
}

impl GlitchTipErrorRequest {
    pub fn new(
        page: String,
        form: String,
        full_url: String,
        bamboo_error: BambooError,
    ) -> Self {
        Self {
            page,
            form,
            full_url,
            bamboo_error,
        }
    }
}

use serde::{Deserialize, Serialize};

#[cfg(feature = "backend")]
use bamboo_common_backend_macros::Responder;

#[derive(Debug, Deserialize, Serialize, Clone, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "backend", derive(Responder))]
pub struct DependencyDetails {
    pub name: String,
    pub authors: String,
    pub repository: String,
    pub license: String,
    pub description: String,
}

impl PartialOrd for DependencyDetails {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DependencyDetails {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

#[cfg(feature = "backend")]
impl DependencyDetails {
    #[must_use]
    pub fn new(
        authors: impl Into<String>,
        name: impl Into<String>,
        repository: impl Into<String>,
        license: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            authors: authors.into(),
            repository: repository.into(),
            license: license.into(),
            description: description.into(),
        }
    }
}

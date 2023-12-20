use bamboo_entities::prelude::FreeCompany;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct FreeCompanies {
    pub free_companies: Vec<FreeCompany>,
}

impl From<Vec<FreeCompany>> for FreeCompanies {
    fn from(value: Vec<FreeCompany>) -> Self {
        Self {
            free_companies: value,
        }
    }
}

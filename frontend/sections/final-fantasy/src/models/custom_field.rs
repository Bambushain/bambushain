use bamboo_entities::prelude::CustomCharacterField;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct CustomCharacterFields {
    pub fields: Vec<CustomCharacterField>,
}

impl From<Vec<CustomCharacterField>> for CustomCharacterFields {
    fn from(value: Vec<CustomCharacterField>) -> Self {
        Self { fields: value }
    }
}

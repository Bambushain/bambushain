use bamboo_entities::prelude::*;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct CrafterForCharacter {
    pub crafter: Vec<Crafter>,
}

impl From<Vec<Crafter>> for CrafterForCharacter {
    fn from(value: Vec<Crafter>) -> Self {
        Self { crafter: value }
    }
}

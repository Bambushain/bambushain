use bamboo_entities::prelude::*;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct FighterForCharacter {
    pub fighter: Vec<Fighter>,
}

impl From<Vec<Fighter>> for FighterForCharacter {
    fn from(value: Vec<Fighter>) -> Self {
        Self { fighter: value }
    }
}

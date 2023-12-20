use bamboo_entities::prelude::*;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct CharacterHousingForCharacter {
    pub character_housing: Vec<CharacterHousing>,
}

impl From<Vec<CharacterHousing>> for CharacterHousingForCharacter {
    fn from(value: Vec<CharacterHousing>) -> Self {
        Self {
            character_housing: value,
        }
    }
}

use bamboo_entities::prelude::*;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct MyCharacters {
    pub character: Vec<Character>,
}

impl From<Vec<Character>> for MyCharacters {
    fn from(value: Vec<Character>) -> Self {
        Self { character: value }
    }
}

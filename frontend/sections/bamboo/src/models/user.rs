use bamboo_entities::prelude::*;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct Users {
    pub users: Vec<WebUser>,
}

impl From<Vec<WebUser>> for Users {
    fn from(value: Vec<WebUser>) -> Self {
        Self { users: value }
    }
}

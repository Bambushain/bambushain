use bamboo_entities::prelude::*;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct EventRange {
    pub events: Vec<Event>,
}

impl From<Vec<Event>> for EventRange {
    fn from(value: Vec<Event>) -> Self {
        Self { events: value }
    }
}

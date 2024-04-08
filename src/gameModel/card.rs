use std::collections::HashSet;
use crate::gameModel::item::Item;

#[derive(Default, Copy, Clone)]
pub struct Card {
    pub items: HashSet<Item>,
}

impl Copy for Card {

}

#[derive(Default)]
pub struct CardBuilder {
    items: HashSet<Item>,
}

impl CardBuilder {

    fn new() -> Self {
        CardBuilder{
            items: HashSet::new()
        }
    }

    pub fn with_items(&mut self, items: HashSet<Item>) -> *mut CardBuilder {
        self.items = items;
        self
    }

    pub fn build(&self) -> Card {
        Card {
            items: self.items.to_vec()
        }
    }
}
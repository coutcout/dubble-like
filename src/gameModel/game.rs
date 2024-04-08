use crate::gameModel::card::Card;

#[derive(Default)]
pub struct Game {
    pub(crate) cards: Vec<Card>,
}

impl Game {
    pub fn builder() -> GameBuilder {
        GameBuilder::default()
    }
}

#[derive(Default)]
pub struct GameBuilder {
    cards: Vec<Card>,
}

impl GameBuilder {
    pub fn new() -> Self {
        GameBuilder {
            cards: Vec::new()
        }
    }

    pub fn with_cards(mut self, cards: Vec<Card>) -> Self {
        self.cards = cards;
        self
    }

    pub fn build(self) -> Game {
        Game {
            cards: Vec::new()
        }
    }
}
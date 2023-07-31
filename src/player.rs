use crate::{card::Card, strategies::Strategy};

pub struct Player {
    name: String,
    cards: Vec<Card>,
    points: Vec<Card>,
    strategy: Box<dyn Strategy>,
}

impl Player {
    pub fn new(name: String, strategy: Box<dyn Strategy>) -> Self {
        Self {
            name,
            cards: Vec::new(),
            points: Vec::new(),
            strategy,
        }
    }

    pub fn clear_hand(&mut self) {
        self.cards.clear();
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }
}

use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Spade,
    Heart,
    Club,
    Diamond,
    Gray,
}

pub struct Card {
    suit: Suit,
    value: u32,
}

impl Card {
    pub fn new(suit: Suit, value: u32) -> Self {
        Self { suit, value }
    }
}

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        use Suit::*;

        let mut cards: Vec<Card> = Vec::new();
        for suit in [Spade, Heart, Club, Diamond] {
            for value in 1..=10 {
                cards.push(Card::new(suit, value));
            }
        }
        for suit in [Gray] {
            for value in 1..=20 {
                cards.push(Card::new(suit, value));
            }
        }

        cards.shuffle(&mut rand::thread_rng());

        Self { cards }
    }

    pub fn count(&self) -> usize {
        self.cards.len()
    }

    pub fn remove(&mut self, value: u32) {
        self.cards
            .retain(|card| card.suit == Suit::Gray || card.value != value);
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

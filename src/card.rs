use std::fmt::Display;

use rand::{seq::SliceRandom, Rng};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Spade,
    Heart,
    Club,
    Diamond,
    Payoo,
}

impl Suit {
    pub fn random() -> Suit {
        let id = rand::thread_rng().gen_range(1..=4);
        match id {
            1 => Suit::Spade,
            2 => Suit::Heart,
            3 => Suit::Diamond,
            4 => Suit::Club,
            _ => unreachable!(),
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = match self {
            Suit::Spade => "Pique",
            Suit::Heart => "Coeur",
            Suit::Club => "Trèfle",
            Suit::Diamond => "Carreau",
            Suit::Payoo => "Payoo",
        }
        .to_owned();
        write!(f, "{display}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    suit: Suit,
    value: u32,
}

impl Card {
    pub fn new(suit: Suit, value: u32) -> Self {
        Self { suit, value }
    }

    pub fn suit(&self) -> Suit {
        self.suit
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn winner(trick: &[Card]) -> usize {
        let first_suit = trick[0].suit();
        trick
            .iter()
            .enumerate()
            .filter(|(_, card)| card.suit() == first_suit)
            .max_by(|(_, card1), (_, card2)| card1.cmp(card2))
            .unwrap()
            .0
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{val} de {suit}", val = self.value, suit = self.suit)
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
        for suit in [Payoo] {
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
            .retain(|card| card.suit == Suit::Payoo || card.value != value);
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::{Card, Suit};

    #[test]
    fn test_card_display() {
        let card = Card::new(Suit::Heart, 2);
        assert_eq!("2 de Coeur", format!("{card}"));
    }
}

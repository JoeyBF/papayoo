use std::ops::Range;

use crate::{
    card::{Card, Suit},
    strategies::{NextMoveData, PassCardsData, Strategy, StrategyInitData},
};

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

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn init_strategy(&mut self, data: StrategyInitData) {
        self.strategy.init(data)
    }

    pub fn clear_hand(&mut self) {
        self.cards.clear();
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
        self.cards.sort();
    }

    pub fn add_cards(&mut self, cards: Vec<Card>) {
        self.cards.extend(cards.into_iter());
        self.cards.sort();
    }

    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    pub fn add_points(&mut self, trick: Vec<Card>) {
        self.points.extend(trick.into_iter())
    }

    pub fn count_points(&self, trump: Suit) -> u32 {
        self.points
            .iter()
            .filter_map(|card| {
                if card.suit() == Suit::Payoo {
                    Some(card.value())
                } else if card.suit() == trump && card.value() == 7 {
                    Some(40)
                } else {
                    None
                }
            })
            .sum()
    }

    pub fn pass_cards(&mut self, nb_to_choose: usize) -> Vec<Card> {
        self.cards.sort();
        let data = PassCardsData::new(nb_to_choose, &self.cards);
        let mut pass_indices = self.strategy.pass_cards(data);
        pass_indices.sort_unstable_by(|a, b| a.cmp(b).reverse());
        pass_indices
            .into_iter()
            .map(|index| self.cards.remove(index))
            .collect()
    }

    pub fn next_move(&mut self, current_trick: &[Card]) -> Card {
        let data = NextMoveData::new(
            &self.cards,
            self.allowed_range(current_trick.get(0).map(Card::suit)),
        );
        let card_idx = self.strategy.next_move(data);
        self.cards.remove(card_idx)
    }

    fn allowed_range(&self, first_suit: Option<Suit>) -> Range<usize> {
        if let Some(first_suit) = first_suit {
            // S,S,H,H,H,H,C,D,D,D,P,P
            // first_suit = H
            // first = 2
            // last = 12 - 6 = 6
            // allowed = self.cards[2..6]
            if let Some(first_allowed_idx) =
                self.cards.iter().position(|card| card.suit() == first_suit)
            {
                let last_allowed_idx = self.cards.len()
                    - self
                        .cards
                        .iter()
                        .rev()
                        .position(|card| card.suit() == first_suit)
                        .unwrap();

                Some(first_allowed_idx..last_allowed_idx)
            } else {
                None
            }
        } else {
            None
        }
        .unwrap_or_else(|| 0..self.cards.len())
    }
}

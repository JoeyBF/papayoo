use std::cmp::Reverse;

use itertools::Itertools;

use super::{NextMoveData, PassCardsData, Strategy, StrategyInitData};
use crate::card::Suit;

pub struct SimpleStrategy {
    trump: Option<Suit>,
}

impl SimpleStrategy {
    pub fn new() -> Self {
        Self { trump: None }
    }
}

impl Strategy for SimpleStrategy {
    fn name(&self) -> &str {
        "SimpleStrategy"
    }

    fn init(&mut self, data: StrategyInitData) {
        match data {
            StrategyInitData::Init2 { trump } => self.trump = Some(trump),
            _ => {}
        }
    }

    fn pass_cards(&mut self, data: PassCardsData) -> Vec<usize> {
        data.current_hand
            .iter()
            .enumerate()
            .sorted_by_cached_key(|(_, card)| {
                // Sort the cards in (non-Payoo, Payoo), and sort each group by decreasing value
                (card.suit() == Suit::Payoo, Reverse(card.value()))
            })
            .map(|(idx, _)| idx)
            .take(data.nb_to_choose)
            .collect()
    }

    fn next_move(&mut self, data: NextMoveData) -> usize {
        data.allowed_range.start
    }

    fn end_round(&mut self) {}
}

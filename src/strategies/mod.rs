#![allow(dead_code)]

use std::ops::Range;

use crate::card::{Card, Suit};

mod simple;
mod trivial;

pub use simple::SimpleStrategy;
pub use trivial::TrivialStrategy;

pub enum StrategyInitData {
    Init1 { nb_of_players: usize },
    Init2 { trump: Suit },
}

pub struct PassCardsData<'a> {
    current_hand: &'a [Card],
    nb_to_choose: usize,
}

impl<'a> PassCardsData<'a> {
    pub fn new(current_hand: &'a [Card], nb_to_choose: usize) -> Self {
        Self {
            current_hand,
            nb_to_choose,
        }
    }
}

pub struct NextMoveData<'a> {
    current_hand: &'a [Card],
    allowed_range: Range<usize>,
}

impl<'a> NextMoveData<'a> {
    pub fn new(current_hand: &'a [Card], allowed_range: Range<usize>) -> Self {
        Self {
            current_hand,
            allowed_range,
        }
    }
}

pub trait Strategy {
    /// Name of the strategy, for logging purposes.
    fn name(&self) -> &str;

    /// Initialize strategy.
    fn init(&mut self, data: StrategyInitData);

    /// Choose which cards to pass to the next player at the beginning of a game.
    fn pass_cards(&mut self, data: PassCardsData) -> Vec<usize>;

    /// Choose which card to play.
    fn next_move(&mut self, data: NextMoveData) -> usize;

    /// Signal the end of a round, prepare for a new one.
    fn end_round(&mut self);
}

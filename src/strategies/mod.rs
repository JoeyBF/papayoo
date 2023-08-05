use std::ops::Range;

use crate::card::{Card, Suit};

mod trivial;

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
    pub fn new(nb_to_choose: usize, current_hand: &'a [Card]) -> Self {
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
    /// Initialize strategy
    fn init(&mut self, data: StrategyInitData);

    /// Strategy that chooses which cards to pass to the next player at the beginning of a game.
    fn pass_cards(&mut self, data: PassCardsData) -> Vec<usize>;

    /// Strategy that chooses which card to play.
    fn next_move(&mut self, data: NextMoveData) -> usize;
}

use crate::card::Card;

pub trait Strategy {
    fn next_move(&mut self, cards: &[Card]) -> Card;
}

pub struct GreedyStrategy;

impl Strategy for GreedyStrategy {
    fn next_move(&mut self, cards: &[Card]) -> Card {
        todo!()
    }
}

pub struct TrivialStrategy;

impl Strategy for TrivialStrategy {
    fn next_move(&mut self, cards: &[Card]) -> Card {
        todo!()
    }
}

fn foo() {
    let mut s = GreedyStrategy;

    s.next_move()
}

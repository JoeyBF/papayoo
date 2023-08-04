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

    pub fn init_strategy(&mut self, data: StrategyInitData) {
        self.strategy.init(data)
    }

    pub fn clear_hand(&mut self) {
        self.cards.clear();
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn add_cards(&mut self, cards: Vec<Card>) {
        self.cards.extend(cards.into_iter());
        self.cards.sort();
    }

    pub fn pass_cards(&mut self, nb_to_choose: usize) -> Vec<Card> {
        let data = PassCardsData::new(nb_to_choose, &self.cards);
        let mut pass_indices = self.strategy.pass_cards(data);
        pass_indices.sort_unstable_by(|a, b| a.cmp(b).reverse());
        pass_indices
            .into_iter()
            .map(|index| self.cards.swap_remove(index))
            .collect()
    }

    pub fn next_move(&mut self, current_trick: &[Card]) -> Card {
        let strategy = &mut self.strategy;
        let data = if current_trick.is_empty() {
            NextMoveData::new(&self.cards, &self.cards)
        } else {
            NextMoveData::new(&self.cards, self.allowed_cards(current_trick[0].suit()))
        };
        let card_idx = strategy.next_move(data);
        self.cards.swap_remove(card_idx)
    }

    fn allowed_cards(&self, first_suit: Suit) -> &[Card] {
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

            &self.cards[first_allowed_idx..last_allowed_idx]
        } else {
            &self.cards
        }
    }
}

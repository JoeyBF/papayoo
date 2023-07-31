use crate::{
    card::{Card, Deck},
    player::Player,
};

pub struct Game {
    players: Vec<Player>,
    current_trick: Vec<Card>,
    dealer: usize,
    turn: u32,
}

impl Game {
    pub fn new(&self, mut players: Vec<Player>) -> Self {
        let nb_of_players = players.len();

        if nb_of_players > 8 {
            panic!("Jouez a autre chose.")
        }

        let mut deck = Deck::new();

        let mut to_remove = 1;
        while deck.count() % nb_of_players != 0 {
            deck.remove(to_remove);
            to_remove += 1;
        }
        let cards_each = deck.count() / nb_of_players;

        // TODO: Implement special dealing rules
        for player in players.iter_mut() {
            player.clear_hand();
            for _ in 0..cards_each {
                player.add_card(deck.draw().unwrap());
            }
        }

        Self {
            players,
            current_trick: Vec::new(),
            dealer: 0,
            turn: 1,
        }
    }

    pub fn play_turn(&mut self) {
        todo!()
    }
}

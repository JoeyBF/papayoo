use crate::{
    card::{Deck, Suit},
    player::Player,
    table::Table,
};

pub struct Game {
    players: Table<Player>,
    trump: Suit,
    turn: u32,
}

impl Game {
    pub fn new(players: Vec<Player>) -> Self {
        let mut players = Table::new(players);
        let nb_of_players = players.len();

        assert!(
            nb_of_players >= 3 && nb_of_players <= 8,
            "Jouez à autre chose"
        );

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

        let nb_to_choose = match nb_of_players {
            3 => 5,
            4 => 5,
            5 => 4,
            6 => 3,
            7 => 3,
            8 => 3,
            _ => unreachable!(),
        };
        let mut cards_passed = players
            .iter_mut()
            .map(|player| player.pass_cards(nb_to_choose))
            .collect::<Vec<_>>();
        for player in players.iter_mut_from(1) {
            player.add_cards(cards_passed.remove(0));
        }

        Self {
            players,
            trump: Suit::random(),
            turn: 1,
        }
    }

    pub fn play_turn(&mut self) {
        let mut current_trick = Vec::new();
        for player in self.players.iter_mut() {
            current_trick.push(player.next_move(&current_trick));
        }
        todo!()
    }
}

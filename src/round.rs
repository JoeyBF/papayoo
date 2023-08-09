use itertools::Itertools;

use crate::{
    card::{Card, Deck, Suit},
    player::Player,
    strategies::StrategyInitData,
    table::Table,
};

pub struct Round {
    players: Table<Player>,
    trump: Suit,
    turn: u32,
}

impl Round {
    pub fn new(players: Vec<Player>) -> Self {
        let mut players = Table::new(players);
        let nb_of_players = players.len();

        assert!(
            nb_of_players >= 3 && nb_of_players <= 8,
            "Jouez à autre chose"
        );

        for player in players.iter_mut() {
            player.init_strategy(StrategyInitData::Init1 { nb_of_players })
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

        for player in players.iter() {
            let cards = player.cards().iter().map(Card::to_string).join(", ");
            println!("Joueur {name} a: {cards}", name = player.name());
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

        for player in players.iter() {
            let cards = player.cards().iter().map(Card::to_string).join(", ");
            println!("Joueur {name} a maintenant: {cards}", name = player.name());
        }

        for (idx, cards) in cards_passed.iter().enumerate() {
            let idx = idx + 1;
            let cards = cards.iter().map(Card::to_string).join(", ");
            println!("Joueur {idx} donne: {cards}");
        }

        for player in players.iter_mut_from(1) {
            player.add_cards(cards_passed.remove(0));
        }

        for player in players.iter() {
            let cards = player.cards().iter().map(Card::to_string).join(", ");
            println!("Joueur {name} a maintenant: {cards}", name = player.name());
        }

        let trump = Suit::random();
        println!("La carte à 40 points est le: {}", Card::new(trump, 7));

        for player in players.iter_mut() {
            player.init_strategy(StrategyInitData::Init2 { trump })
        }

        Self {
            players,
            trump,
            turn: 1,
        }
    }

    pub fn play_round(&mut self) {
        while self.players[0].cards().len() > 0 {
            self.play_turn();
        }
    }

    pub fn play_turn(&mut self) {
        let mut trick = Vec::new();
        for player in self.players.iter_mut() {
            let next_card = player.next_move(&trick);
            println!(
                "Joueur {name} joue la carte {next_card}",
                name = player.name()
            );
            trick.push(next_card);
        }
        let winner_idx = Card::winner(&trick);
        println!(
            "Le gagnant est: Joueur {name}",
            name = self.players[winner_idx].name()
        );
        self.players[winner_idx].add_points(trick);
        self.players.set_dealer(winner_idx);
        self.print_results();

        self.turn += 1;
    }

    fn print_results(&self) {
        if self.turn == 1 {
            let title = self.players.iter_from(0).map(Player::name).join(" | ");
            let separator = "-".repeat(title.chars().count());
            println!("{title}");
            println!("{separator}");
        }
        let scores = self
            .players
            .iter_from(0)
            .map(|player| player.count_points(self.trump))
            .join(" | ");
        println!("{scores}");
    }
}

#[cfg(test)]
mod tests {
    use super::Round;
    use crate::{card::Suit, player::Player, strategies::TrivialStrategy};

    #[test]
    fn test_remove_1_7() {
        let players = (1..=7)
            .map(|i| Player::new(i.to_string(), Box::new(TrivialStrategy)))
            .collect();
        let game = Round::new(players);

        for player in game.players.iter() {
            for card in player
                .cards()
                .iter()
                .filter(|card| card.suit() != Suit::Payoo)
            {
                assert_ne!(card.value(), 1);
            }
        }
    }

    #[test]
    fn test_remove_1_8() {
        let players = (1..=8)
            .map(|i| Player::new(i.to_string(), Box::new(TrivialStrategy)))
            .collect();
        let game = Round::new(players);

        for player in game.players.iter() {
            for card in player
                .cards()
                .iter()
                .filter(|card| card.suit() != Suit::Payoo)
            {
                assert_ne!(card.value(), 1);
            }
        }
    }

    #[test]
    #[should_panic(expected = "Jouez à autre chose")]
    fn test_more_than_8() {
        let players = (1..=9)
            .map(|i| Player::new(i.to_string(), Box::new(TrivialStrategy)))
            .collect();
        Round::new(players);
    }
}

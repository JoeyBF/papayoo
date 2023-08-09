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
    pub fn new(mut players: Table<Player>) -> Self {
        let nb_of_players = players.len();
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

        let trump = Suit::random();

        for player in players.iter_mut() {
            player.init_strategy(StrategyInitData::Init2 { trump })
        }

        Self {
            players,
            trump,
            turn: 1,
        }
    }

    pub fn trump(&self) -> Suit {
        self.trump
    }

    pub fn play_round(mut self) -> Vec<Player> {
        while self.players[0].cards().len() > 0 {
            self.play_turn();
        }
        self.players.iter_mut().for_each(Player::end_round);
        self.players.into_inner()
    }

    pub fn play_turn(&mut self) {
        let mut trick = Vec::new();
        for player in self.players.iter_mut() {
            let next_card = player.next_move(&trick);
            trick.push(next_card);
        }
        let winner_idx = Card::winner(&trick);
        self.players[winner_idx].add_points(trick);
        self.players.set_dealer(winner_idx);

        self.turn += 1;
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

        for player in game.players.into_inner() {
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

        for player in game.players.into_inner() {
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

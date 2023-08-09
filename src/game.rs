use itertools::Itertools;

use crate::{player::Player, round::Round, strategies::StrategyInitData, table::Table};

pub struct Game {
    players: Vec<Player>,
    points: Vec<Vec<u32>>,
    dealer: usize,
}

impl Game {
    pub fn new(mut players: Vec<Player>) -> Self {
        let nb_of_players = players.len();

        assert!(
            nb_of_players >= 3 && nb_of_players <= 8,
            "Jouez à autre chose"
        );

        for player in players.iter_mut() {
            player.clear_hand();
            player.init_strategy(StrategyInitData::Init1 { nb_of_players })
        }

        Self {
            players,
            points: Vec::new(),
            dealer: 0,
        }
    }

    pub fn play_game(mut self, nb_of_rounds: usize) {
        for i in 0..nb_of_rounds {
            if i == 0 {
                let title = self.players.iter().map(Player::name).join(" | ");
                let separator = "-".repeat(title.chars().count());
                println!("{title}");
                println!("{separator}");
            }

            for player in self.players.iter_mut() {
                player.clear_hand();
                player.clear_points();
            }

            let mut table = Table::new(self.players);
            table.set_dealer(self.dealer);
            self.dealer += 1;

            let round = Round::new(table);
            let trump = round.trump();
            self.players = round.play_round();

            let new_points = {
                let mut previous_points = if let Some(v) = self.points.last() {
                    v.clone()
                } else {
                    vec![0; self.players.len()]
                };

                for (previous_score, player) in previous_points.iter_mut().zip(self.players.iter())
                {
                    *previous_score += player.count_points(trump);
                }

                previous_points
            };
            assert_eq!(new_points.iter().sum::<u32>() % 250, 0);
            self.points.push(new_points);
        }
        let results = self
            .points
            .iter()
            .map(|round_points| round_points.iter().join(" | "))
            .join("\n");
        println!("{results}");
    }
}

use card::Deck;
use player::Player;
use strategies::GreedyStrategy;

use crate::game::Game;

mod card;
mod game;
mod player;
mod strategies;

fn main() {
    let player_un = Player::new("1".to_owned(), Box::new(GreedyStrategy));
}

use player::Player;
use strategies::TrivialStrategy;

use crate::game::Game;

mod card;
mod game;
mod player;
mod strategies;
mod table;

fn main() {
    let player_un = Player::new("1".to_owned(), Box::new(TrivialStrategy));
    let player_deux = Player::new("2".to_owned(), Box::new(TrivialStrategy));
    let player_trois = Player::new("3".to_owned(), Box::new(TrivialStrategy));
    let player_quatre = Player::new("4".to_owned(), Box::new(TrivialStrategy));
    let mut game = Game::new(vec![player_un, player_deux, player_trois, player_quatre]);
    game.play_turn();
}

use game::Game;
use player::Player;
use strategies::{SimpleStrategy, TrivialStrategy};

mod card;
mod game;
mod player;
mod round;
mod strategies;
mod table;

fn main() {
    let player_un = Player::new("Smarter1".to_owned(), Box::new(SimpleStrategy::new()));
    let player_deux = Player::new("2".to_owned(), Box::new(TrivialStrategy));
    let player_trois = Player::new("3".to_owned(), Box::new(TrivialStrategy));
    let player_quatre = Player::new("4".to_owned(), Box::new(TrivialStrategy));
    let player_cinq = Player::new("5".to_owned(), Box::new(TrivialStrategy));
    let game = Game::new(vec![
        player_un,
        player_deux,
        player_trois,
        player_quatre,
        player_cinq,
    ]);
    game.play_game(100_000);
}

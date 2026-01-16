mod game;

use crate::game::Game;

pub fn main() {
  let game = Game::new(game::PlayerCount::Two);
  println!("{:?}", game);
}

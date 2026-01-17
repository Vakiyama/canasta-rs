use crate::game;

fn get_moves() {
  todo!()
}

fn make_move() {
  todo!()
}

// now we must make a set of moves from a game
// we know the active player through the turn counter on game
// a player can do up to 3 things

enum Move {
  EndTurn(game::Card), // card we throw away to end turn
  DrawFromPile,        //
  DrawTable,
  MakeNewMeld(game::Meld),
  AddToMeld(game::Meld),
}

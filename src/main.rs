fn main() {
  println!("Hello, world!");
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Suit {
  Hearts,
  Spades,
  Clubs,
  Diamonds,
}

mod numerical_rank {
  #[derive(Debug, PartialEq, Eq, Clone, Copy)]
  pub struct NumericalRank(u8);

  impl NumericalRank {
    pub fn new(number: u8) -> Option<NumericalRank> {
      match number {
        1..=13 => Some(NumericalRank(number)),
        _ => None,
      }
    }
  }

  impl From<NumericalRank> for u8 {
    fn from(rank: NumericalRank) -> u8 {
      rank.0
    }
  }
}

#[derive(Debug)]
enum Face {
  Jack,
  Queen,
  King,
}

#[derive(Debug, Clone)]
struct CardData {
  suit: Suit,
  rank: numerical_rank::NumericalRank,
}

#[derive(Debug, Clone)]
enum Card {
  Card(CardData),
  Joker,
}

enum OrderError {
  Suit,
  Rank,
}

enum Direction {
  Increasing,
  Decreasing,
}

impl Card {
  fn check_neighbour(&self, card: &Card, direction: &Direction) -> Result<(), OrderError> {
    self.check_suit(card)?;
    self.check_order(card, direction)?;
    Ok(())
  }

  fn check_suit(&self, card: &Card) -> Result<(), OrderError> {
    match (self, card) {
      (Card::Joker, Card::Joker) => Err(OrderError::Suit),
      (Card::Joker, _) => Ok(()),
      (_, Card::Joker) => Ok(()),
      (Card::Card(a), Card::Card(b)) => {
        if a.suit == b.suit {
          Ok(())
        } else {
          Err(OrderError::Suit)
        }
      }
    }
  }

  fn check_order(&self, card: &Card, direction: &Direction) -> Result<(), OrderError> {
    // subtract self from card
    // desired size is 1 or 13 if Decreasing, -1 or -13 if increasing

    match (self, card) {
      (Card::Joker, Card::Joker) => Err(OrderError::Suit),
      (Card::Joker, _) => Ok(()),
      (_, Card::Joker) => Ok(()),
      (Card::Card(a), Card::Card(b)) => {
        let diff = u8::from(a.rank) as i8 - u8::from(b.rank) as i8;

        match (diff, direction) {
          (1 | -12, Direction::Decreasing) | (-1 | 12, Direction::Increasing) => Ok(()),
          (_, _) => Err(OrderError::Rank),
        }
      }
    }
  }
}

struct Hand(Vec<Card>);

// sets have some critical invariants;
// they must be min 3 long
// they must be ordered and of the same suit (except for 2* as they are small jokers)
// they are ranged from ace - 2 - .. - king - ace
#[derive(Debug)]
struct Set(Vec<Card>);

enum SetError {
  Size(usize),
  Order(OrderError),
}

impl From<OrderError> for SetError {
  fn from(value: OrderError) -> Self {
    SetError::Order(value)
  }
}

impl Set {
  pub fn new(mut initial: Vec<Card>) -> Result<Set, SetError> {
    let len = initial.len();

    if len < 3 {
      return Err(SetError::Size(len));
    };

    // choose middle card, check_neighbours in both dirs
    let middle_index = len / 2;
    let middle_card = &initial[middle_index].to_owned();

    let initial_copy: &Vec<Card> = &initial.to_owned();

    let second_half = &initial_copy[middle_index + 1..];
    let first_half: &mut [Card] = &mut initial[..middle_index];

    first_half.reverse();

    check_neighbours(middle_card, first_half, &Direction::Decreasing)?;
    check_neighbours(middle_card, second_half, &Direction::Increasing)?;

    Ok(Set(initial))
  }
}

fn check_neighbours(
  current: &Card,
  rest: &[Card],
  direction: &Direction,
) -> Result<(), OrderError> {
  match rest {
    [] => Ok(()),
    [first, rest @ ..] => {
      current.check_neighbour(first, direction)?;
      check_neighbours(first, rest, direction)
    }
  }
}

struct Player {}

struct Game {
  players: (Player, Player),
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_check_suit_ok() {
    let ace_hearts = Card::Card(CardData {
      suit: Suit::Hearts,
      rank: numerical_rank::NumericalRank::new(1).unwrap(),
    });

    todo!()
  }
}

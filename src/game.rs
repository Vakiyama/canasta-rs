use std::vec::Vec;

use rand::{rng, seq::SliceRandom};

use crate::game::numerical_rank::NumericalRank;

const HAND_SIZE: usize = 11;

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

#[derive(Debug)]
enum Face {
  Jack,
  Queen,
  King,
}

mod numerical_rank {
  #[derive(Debug, PartialEq, Eq, Clone, Copy)]
  pub struct NumericalRank(u8);

  impl NumericalRank {
    pub fn new(number: u8) -> Option<Self> {
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

#[derive(Debug, Clone, PartialEq)]
struct CardData {
  suit: Suit,
  rank: NumericalRank,
}

#[derive(Debug, Clone, PartialEq)]
enum Card {
  Card(CardData),
  Joker,
}

#[derive(Debug, PartialEq)]
enum OrderError {
  Suit,
  Rank,
}

enum Direction {
  Increasing,
  Decreasing,
}

impl Card {
  fn new(suit: Suit, rank: u8) -> Option<Self> {
    println!("{:?}, {:?}", suit, rank);
    NumericalRank::new(rank).map(|rank| Card::Card(CardData { suit, rank }))
  }

  pub fn new_face(suit: Suit, face: Face) -> Self {
    let rank = match face {
      Face::Jack => NumericalRank::new(11).unwrap(),
      Face::Queen => NumericalRank::new(12).unwrap(),
      Face::King => NumericalRank::new(13).unwrap(),
    };

    Card::Card(CardData { suit, rank })
  }

  pub fn new_ace(suit: Suit) -> Self {
    Card::Card(CardData {
      suit,
      rank: NumericalRank::new(1).unwrap(),
    })
  }

  fn new_joker() -> Self {
    Card::Joker
  }

  fn check_neighbour(&self, card: &Card, direction: &Direction) -> Result<(), OrderError> {
    self.check_suit(card)?;
    self.check_rank(card, direction)?;
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

  fn check_rank(&self, card: &Card, direction: &Direction) -> Result<(), OrderError> {
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

// melds have some critical invariants;
// they must be min 3 long
// they must be ordered and of the same suit (except for 2* as they are small jokers)
// they are ranged from ace - 2 - .. - king - ace
#[derive(Debug, PartialEq)]
struct Meld(Vec<Card>);

#[derive(Debug, PartialEq)]
enum SetError {
  Size(usize),
  Order(OrderError),
}

impl From<OrderError> for SetError {
  fn from(value: OrderError) -> Self {
    SetError::Order(value)
  }
}

impl Meld {
  pub fn new(mut initial: Vec<Card>) -> Result<Meld, SetError> {
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

    Ok(Meld(initial))
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

// game consists of 2 teams
// each team has 1..=3 players
// the team has a played set of cards
// each player has their own hand and a flag for having picked up the refill
// the game has the main deck,
// 0..=2 refills,
// the table deck

#[derive(Debug)]
struct Player {
  refill_used: bool,
  hand: Cards,
}

impl Player {
  fn new(deck: &mut Cards) -> Self {
    Player {
      refill_used: false,
      hand: deck.draw_n(HAND_SIZE),
    }
  }
}

#[derive(Debug)]
struct Team {
  players: Vec<Player>,
  melds: Vec<Meld>,
}

impl Team {
  fn new(deck: &mut Cards, num_players: usize) -> Self {
    Team {
      players: { 0..=num_players }.map(|_| Player::new(deck)).collect(),
      melds: vec![],
    }
  }
}

#[derive(Debug)]
struct Cards(Vec<Card>);

impl Cards {
  fn shuffle(&mut self) {
    let mut rng = rng();
    self.0.shuffle(&mut rng);
  }

  fn draw_n(&mut self, n: usize) -> Cards {
    let cards: Vec<_> = self.0.drain(0..=n).collect();

    Cards(cards)
  }
}

#[derive(Debug)]
pub struct Game {
  teams: Vec<Team>,
  refills: (Option<Cards>, Option<Cards>),
  deck: Cards,
  table: Cards,
}

pub enum PlayerCount {
  Two,
  Three,
  Four,
  Six,
}

impl Game {
  pub fn new(player_count: PlayerCount) -> Self {
    let mut deck = Cards(new_deck().into_iter().chain(new_deck()).collect());
    deck.shuffle();

    let teams = match player_count {
      PlayerCount::Two => vec![Team::new(&mut deck, 1), Team::new(&mut deck, 1)],
      PlayerCount::Three => vec![
        Team::new(&mut deck, 1),
        Team::new(&mut deck, 1),
        Team::new(&mut deck, 1),
      ],
      PlayerCount::Four => vec![Team::new(&mut deck, 2), Team::new(&mut deck, 2)],
      PlayerCount::Six => todo!(),
    };

    Game {
      teams,
      refills: (Some(deck.draw_n(HAND_SIZE)), Some(deck.draw_n(HAND_SIZE))),
      deck,
      table: Cards(vec![]),
    }
  }
}

fn new_deck() -> Vec<Card> {
  [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs]
    .iter()
    .flat_map(|suit| { 1..=13 }.map(|rank| Card::new(suit.clone(), rank).unwrap()))
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn make_new_deck() {
    let deck = new_deck();
    assert_eq!(deck.len(), 52)
  }

  #[test]
  fn check_suit_ok() {
    let ace_hearts = Card::new(Suit::Hearts, 1).unwrap();
    let two_hearts = Card::new(Suit::Hearts, 2).unwrap();
    let joker = Card::new_joker();

    assert_eq!(ace_hearts.check_suit(&two_hearts), Ok(()));
    assert_eq!(ace_hearts.check_suit(&joker), Ok(()));
  }

  #[test]
  fn check_suit_error() {
    let ace_hearts = Card::new(Suit::Hearts, 1).unwrap();
    let two_spades = Card::new(Suit::Spades, 2).unwrap();

    assert_eq!(ace_hearts.check_suit(&two_spades), Err(OrderError::Suit));
  }

  #[test]
  fn check_rank_ok() {
    let ace_hearts = Card::new(Suit::Hearts, 1).unwrap();
    let two_spades = Card::new(Suit::Spades, 2).unwrap();

    assert_eq!(
      ace_hearts.check_rank(&two_spades, &Direction::Increasing),
      Ok(())
    );

    let three_spades = Card::new(Suit::Spades, 3).unwrap();

    assert_eq!(
      two_spades.check_rank(&three_spades, &Direction::Increasing),
      Ok(())
    );

    let king_spades = Card::new_face(Suit::Spades, Face::King);

    assert_eq!(
      king_spades.check_rank(&ace_hearts, &Direction::Increasing),
      Ok(())
    );
  }

  #[test]
  fn check_rank_error() {
    let ace_hearts = Card::new(Suit::Hearts, 1).unwrap();
    let two_spades = Card::new(Suit::Spades, 2).unwrap();

    assert_eq!(
      ace_hearts.check_rank(&two_spades, &Direction::Decreasing),
      Err(OrderError::Rank)
    );

    let three_spades = Card::new(Suit::Spades, 3).unwrap();

    assert_eq!(
      ace_hearts.check_rank(&three_spades, &Direction::Increasing),
      Err(OrderError::Rank)
    );

    let king_spades = Card::new_face(Suit::Spades, Face::King);

    assert_eq!(
      ace_hearts.check_rank(&king_spades, &Direction::Increasing),
      Err(OrderError::Rank)
    );
  }

  // check recursive set validation
  #[test]
  fn check_set_ok() {
    let ace_spades = Card::new(Suit::Spades, 1).unwrap();
    let two_spades = Card::new(Suit::Spades, 2).unwrap();
    let three_spades = Card::new(Suit::Spades, 3).unwrap();

    let set_vec = vec![ace_spades, two_spades, three_spades];
    let set_vec_copy = set_vec.to_owned();

    assert_eq!(Meld::new(set_vec), Ok(Meld(set_vec_copy)));

    let mut full_set: Vec<_> = { 1..=13 }
      .inspect(|&num| {
        println!("{}", num);
      })
      .map(|num| Card::new(Suit::Diamonds, num).unwrap())
      .collect();

    full_set.push(Card::new_ace(Suit::Diamonds));

    assert!(Meld::new(full_set.to_owned()).is_ok());

    full_set.push(Card::new_ace(Suit::Diamonds));

    assert_eq!(Meld::new(full_set), Err(SetError::Order(OrderError::Rank)));
  }

  #[test]
  fn check_set_rank_err() {
    let ace_spades = Card::new(Suit::Spades, 1).unwrap();
    let two_spades = Card::new(Suit::Spades, 2).unwrap();

    assert_eq!(
      Meld::new(vec![ace_spades.to_owned(), two_spades.to_owned()]),
      Err(SetError::Size(2))
    );

    assert_eq!(
      Meld::new(vec![
        ace_spades.to_owned(),
        two_spades.to_owned(),
        ace_spades.to_owned()
      ]),
      Err(SetError::Order(OrderError::Rank))
    );

    assert_eq!(
      Meld::new(vec![
        two_spades.to_owned(),
        two_spades.to_owned(),
        two_spades.to_owned()
      ]),
      Err(SetError::Order(OrderError::Rank))
    );
  }
}

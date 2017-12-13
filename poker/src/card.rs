
use std::str;
use std::fmt::{Display, Formatter, Result};
use std::slice::Iter;


#[derive(Debug)]
pub struct Card {
  pub suit: Suit,
  pub rank: &'static str
}

#[derive(Debug)]
pub enum Suit {
  Spades,
  Diamonds,
  Clubs,
  Hearts,
}

impl Suit {
    pub fn iter() -> Iter<'static, Suit> {
        static SUITS: [Suit;  4] = [Suit::Spades, Suit::Diamonds, Suit::Clubs, Suit::Hearts];
        SUITS.into_iter()
    }
}

impl Display for Card {
  fn fmt(self: &Card, f: &mut Formatter) -> Result {
    let rank = match self.rank {
      "2" => "Deuce",
      "11" => "Jack",
      "12" => "Queen",
      "13" => "King",
      "14" => "Ace",
      _ => self.rank
    };
    write!(f, "{} of {:?}", rank, self.suit)
  }
}

impl Card {

  pub fn new(rank: u8, suit: Suit) -> Card {
    let r = rank.to_string().to_owned();
    Card{rank: &r, suit: suit}
  }

  pub fn value(self: &Card) -> Vec<u8> {
    if self.rank != "14" {
      vec![self.rank.parse::<u8>().unwrap()]
    } else {
      vec![1, 14]
    }
  }

}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_format_correctly() {
      let card = Card{suit:Suit::Spades, rank:"14"};
      let formatted_card = format!("{}", card);
      assert_eq!(formatted_card, "Ace of Spades");
    }


    #[test]
    fn should_format_correctly2() {
      let card = Card{suit:Suit::Clubs, rank:"2"};
      let formatted_card = format!("{}", card);
      assert_eq!(formatted_card, "Deuce of Clubs");
    }

    #[test]
    fn should_format_correctly3() {
      let card = Card{suit:Suit::Diamonds, rank:"5"};
      let formatted_card = format!("{}", card);
      assert_eq!(formatted_card, "5 of Diamonds");
    }


    #[test]
    fn should_format_correctly4() {
      let card = Card{suit:Suit::Hearts, rank:"13"};
      let formatted_card = format!("{}", card);
      assert_eq!(formatted_card, "King of Hearts");
    }

    #[test]
    fn deuce_should_have_value_of_2() {
      let card = Card{suit:Suit::Hearts, rank:"2"};
      assert_eq!(card.value(), vec![2]);
    }

    #[test]
    fn ace_should_have_value_if_1_or_14() {
      let card = Card{suit:Suit::Hearts, rank:"14"};
      assert_eq!(card.value(), vec![1, 14]);
    }

    #[test]
    fn king_should_have_value_if_13() {
      let card = Card{suit:Suit::Hearts, rank:"13"};
      assert_eq!(card.value(), vec![13]);
    }
}

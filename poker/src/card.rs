
use std::str;
use std::fmt::{Display, Formatter, Result};


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
  fn value(self: &Card) -> u8 {
    self.rank.parse::<u8>().unwrap()
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
    fn ace_should_have_value_of_14() {
      let card = Card{suit:Suit::Hearts, rank:"14"};
      assert_eq!(card.value(), 14);
    }

    #[test]
    fn deuce_should_have_value_of_2() {
      let card = Card{suit:Suit::Hearts, rank:"2"};
      assert_eq!(card.value(), 2);
    }
}
